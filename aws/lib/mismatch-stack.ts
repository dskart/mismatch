import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { CfnParameters } from './cfn-parameters';
import {
    aws_ec2 as ec2,
    Aws,
    aws_ecs as ecs,
    aws_ecr as ecr,
    aws_logs as logs,
    aws_iam as iam,
    aws_servicediscovery as servicediscovery,
    aws_apigatewayv2 as apigatewayv2,
    aws_apigatewayv2_integrations as apigatewayv2_integrations,
} from 'aws-cdk-lib';
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
            ipAddresses: ec2.IpAddresses.cidr('10.0.0.0/16'),
            enableDnsHostnames: true,
            enableDnsSupport: true,
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

        const ecsHostSg = new ec2.SecurityGroup(this, 'EcsHostSecurityGroup', {
            vpc,
            allowAllOutbound: true,
            description: 'Security group for ECS host instances',
        });

        const cluster = new Cluster(this, 'MismatchCluster', {
            vpc: vpc,
            clusterName: 'mismatch-cluster',
            instanceType: params.mismatchInstanceType,
            minCapacity: 0,
            maxCapacity: 1,
            logGroup: logGroup,
            securityGroup: ecsHostSg,
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

        ecsTaskDefinition.addContainer('Container', {
            image: image,
            containerName: 'mismatch-server',
            command: ['serve', '--port', '8080'],
            portMappings: [{ containerPort: 8080, hostPort: 0 }],
            memoryLimitMiB: 900,
            cpu: 2048 / 2,
            logging: ecs.LogDriver.awsLogs({ logGroup: logGroup, streamPrefix: Aws.STACK_NAME }),
            environment,
        });

        const vpcLinkSg = new ec2.SecurityGroup(this, 'VpcLinkSecurityGroup', {
            vpc,
            allowAllOutbound: false,
            description: 'Security group for VPC Link',
        });

        // Allow VPC Link to access ECS service on ephemeral ports (32768-65535)
        ecsHostSg.addIngressRule(
            vpcLinkSg,
            ec2.Port.tcpRange(32768, 65535),
            'Allow inbound from VPC Link on ephemeral ports',
        );

        // Allow VPC Link to make outbound connections to ECS service
        vpcLinkSg.addEgressRule(
            ecsHostSg,
            ec2.Port.tcpRange(32768, 65535),
            'Allow outbound to ECS service on ephemeral ports',
        );

        const ecsService = new ecs.Ec2Service(this, 'Service', {
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
            cloudMapOptions: {
                name: 'mismatch-service',
                dnsRecordType: servicediscovery.DnsRecordType.SRV,
                dnsTtl: cdk.Duration.seconds(60),
            },
            deploymentController: {
                type: ecs.DeploymentControllerType.ECS,
            },
            circuitBreaker: { rollback: true },
            minHealthyPercent: 100,
            maxHealthyPercent: 200,
        });

        const vpcLink = new apigatewayv2.VpcLink(this, 'VpcLink', {
            vpc: vpc,
            subnets: { subnetType: ec2.SubnetType.PRIVATE_WITH_EGRESS },
            securityGroups: [vpcLinkSg],
        });

        const api = new apigatewayv2.HttpApi(this, 'MismatchApi', {
            apiName: 'mismatch-api',
            createDefaultStage: true,
        });

        const integration = new apigatewayv2_integrations.HttpServiceDiscoveryIntegration(
            'DefaultIntegration',
            ecsService.cloudMapService!,
            {
                vpcLink: vpcLink,
            },
        );

        api.addRoutes({
            path: '/{proxy+}',
            methods: [apigatewayv2.HttpMethod.ANY],
            integration: integration,
        });

        new cdk.CfnOutput(this, 'ApiEndpoint', {
            value: api.apiEndpoint,
            description: 'API Gateway endpoint URL',
        });
    }
}
