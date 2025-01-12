import {
    aws_autoscaling as autoscaling,
    aws_ec2 as ec2,
    aws_iam as iam,
    CfnParameter,
    Duration,
    aws_ecs as ecs,
    aws_logs as logs,
} from 'aws-cdk-lib';
import { Construct } from 'constructs';

interface ClusterProps {
    readonly vpc: ec2.IVpc;
    readonly clusterName: string;
    readonly instanceType: CfnParameter;
    readonly minCapacity: number;
    readonly maxCapacity: number;
    readonly logGroup: logs.LogGroup;
    readonly hardwareType: ecs.AmiHardwareType;
    readonly subnetType: ec2.SubnetType;
    readonly spotPrice?: string;
    readonly blockDevices?: autoscaling.BlockDevice[];
}

export class Cluster extends Construct {
    readonly ecsCluster: ecs.Cluster;
    readonly autoScalingGroup: autoscaling.AutoScalingGroup;
    readonly capacityProvider: ecs.AsgCapacityProvider;

    constructor(scope: Construct, id: string, props: ClusterProps) {
        super(scope, id);

        const clusterName = props.clusterName;
        const autoScalingGroupName = props.clusterName + '-' + id + '-' + 'ASG';

        this.ecsCluster = new ecs.Cluster(this, 'Cluster', { vpc: props.vpc, clusterName });
        const autoScalingGroup = new autoscaling.AutoScalingGroup(this, 'AutoscalingGroup', {
            autoScalingGroupName,
            vpc: props.vpc,
            signals: autoscaling.Signals.waitForAll({
                timeout: Duration.minutes(10),
            }),
            maxCapacity: props.maxCapacity,
            minCapacity: props.minCapacity,
            machineImage: ecs.EcsOptimizedImage.amazonLinux2(props.hardwareType, {
                cachedInContext: false,
            }),
            role: new iam.Role(this, 'ASGRole', {
                assumedBy: new iam.ServicePrincipal('ec2.amazonaws.com'),
                managedPolicies: [
                    iam.ManagedPolicy.fromAwsManagedPolicyName('service-role/AmazonEC2RoleforSSM'),
                    iam.ManagedPolicy.fromAwsManagedPolicyName('service-role/AmazonEC2ContainerServiceforEC2Role'),
                ],
            }),
            instanceType: new ec2.InstanceType(props.instanceType.valueAsString),
            vpcSubnets: {
                subnetType: props.subnetType,
            },
            newInstancesProtectedFromScaleIn: true,
            spotPrice: props.spotPrice,
            blockDevices: props.blockDevices,
        });
        autoScalingGroup.userData.addCommands('yum install -y aws-cfn-bootstrap');
        autoScalingGroup.userData.addSignalOnExitCommand(autoScalingGroup);

        this.capacityProvider = new ecs.AsgCapacityProvider(this, 'AsgCapacityProvider', {
            autoScalingGroup,
            enableManagedTerminationProtection: true,
            enableManagedScaling: true,
        });
        this.ecsCluster.addAsgCapacityProvider(this.capacityProvider);
        this.ecsCluster.addDefaultCapacityProviderStrategy([
            {
                capacityProvider: this.capacityProvider.capacityProviderName,
                weight: 1,
            },
        ]);
    }
}
