import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { CfnParameters } from './cfn-parameters';
import { aws_ec2 as ec2, Aws, aws_ecs as ecs, aws_ecr as ecr, aws_logs as logs, aws_iam as iam } from 'aws-cdk-lib';
import { Cluster } from './cluster';

export class MismatchStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);
        const params = new CfnParameters(this);

        const natGatewayProvider = ec2.NatProvider.instanceV2({
            instanceType: new ec2.InstanceType('t3.nano'),
        });
        const vpc = new ec2.Vpc(this, 'VPC', {
            vpcName: 'MismatchVpc',
            ipProtocol: ec2.IpProtocol.DUAL_STACK,
            ipAddresses: ec2.IpAddresses.cidr('10.0.0.0/16'),
            enableDnsHostnames: false,
            enableDnsSupport: false,
            maxAzs: 2,
            natGatewayProvider,
            natGateways: 1,
            subnetConfiguration: [
                {
                    name: 'Public',
                    subnetType: ec2.SubnetType.PUBLIC,
                },
                {
                    name: 'Private',
                    subnetType: ec2.SubnetType.PRIVATE_WITH_EGRESS,
                },
            ],
        });
        cdk.Tags.of(vpc).add('Name', Aws.STACK_NAME);

        const logGroup = new logs.LogGroup(this, 'LogGroup', {
            logGroupName: Aws.STACK_NAME,
            removalPolicy: cdk.RemovalPolicy.DESTROY,
            retention: logs.RetentionDays.ONE_WEEK,
        });

        const image = ecs.ContainerImage.fromEcrRepository(
            ecr.Repository.fromRepositoryName(this, 'EcrRepository', params.ecrRepositoryName.valueAsString),
            params.imageTag.valueAsString,
        );

        const cluster = new Cluster(this, 'MismatchCluster', {
            vpc: vpc,
            clusterName: 'MismatchCluster',
            instanceType: params.mismatchInstanceType,
            minCapacity: 0,
            maxCapacity: 1,
            logGroup: logGroup,
            hardwareType: ecs.AmiHardwareType.STANDARD,
            subnetType: ec2.SubnetType.PRIVATE_WITH_EGRESS,
        });

        const taskRole = new iam.Role(this, 'EcsTaskRole', {
            roleName: 'MismatchTaskRole',
            assumedBy: new iam.ServicePrincipal('ecs-tasks.amazonaws.com'),
        });

        const ecsTaskDefinition = new ecs.Ec2TaskDefinition(this, 'EcsTaskDefinition', {
            family: 'mismatch-task-definition',
            taskRole: taskRole,
            networkMode: ecs.NetworkMode.BRIDGE,
        });

        const environment = {
            AWS_DEFAULT_REGION: Aws.REGION,
        };

        // t3.micro: 2 vCPU, 1 GiB
        ecsTaskDefinition.addContainer('Container', {
            image: image,
            containerName: 'mismatch-server',
            command: ['serve'],
            portMappings: [{ containerPort: 8080, hostPort: 0 }],
            memoryLimitMiB: 900,
            cpu: 2048 / 2,
            logging: ecs.LogDriver.awsLogs({ logGroup: logGroup, streamPrefix: Aws.STACK_NAME }),
            environment,
        });

        new ecs.Ec2Service(this, 'Service', {
            serviceName: 'mismatch-server-service',
            cluster: cluster.ecsCluster,
            taskDefinition: ecsTaskDefinition,
            desiredCount: 1,
            capacityProviderStrategies: [
                {
                    capacityProvider: cluster.capacityProvider.capacityProviderName,
                    weight: 1,
                },
            ],
            deploymentController: {
                type: ecs.DeploymentControllerType.ECS,
            },
            circuitBreaker: { rollback: true },
            minHealthyPercent: 100,
            maxHealthyPercent: 200,
        });
    }
}
