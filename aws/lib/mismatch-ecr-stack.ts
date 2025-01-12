import * as cdk from 'aws-cdk-lib';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import { Construct } from 'constructs';

export class MismatchEcrStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const repository = new ecr.Repository(this, 'MismatchEcrRepo', {
            repositoryName: 'mismatch',
            imageScanOnPush: true,
            lifecycleRules: [
                {
                    description: 'Keep only recent images',
                    maxImageCount: 6,
                    rulePriority: 1,
                },
            ],
            encryption: ecr.RepositoryEncryption.KMS,
            removalPolicy: cdk.RemovalPolicy.DESTROY,
        });

        new cdk.CfnOutput(this, 'EcrRepositoryArn', {
            value: repository.repositoryArn,
            description: 'The ARN of the ECR repository',
        });

        new cdk.CfnOutput(this, 'EcrRepositoryName', {
            value: repository.repositoryName,
            description: 'The name of the ECR repository',
        });
    }
}
