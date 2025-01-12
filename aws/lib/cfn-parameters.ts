import { CfnParameter } from 'aws-cdk-lib';
import { Construct } from 'constructs';

export class CfnParameters {
    readonly ecrRepositoryName: CfnParameter;
    readonly imageTag: CfnParameter;

    readonly mismatchInstanceType: CfnParameter;

    constructor(scope: Construct) {
        this.ecrRepositoryName = new CfnParameter(scope, 'ECRRepositoryName', {
            type: 'String',
            default: 'mismatch',
            description: 'The ecr repository name for the docker images',
        });
        this.ecrRepositoryName.overrideLogicalId('EcrRepositoryName');

        this.imageTag = new CfnParameter(scope, 'ImageTag', {
            type: 'String',
            default: 'latest',
            description: 'image tag for the docker images',
        });
        this.imageTag.overrideLogicalId('ImageTag');

        this.mismatchInstanceType = new CfnParameter(scope, 'MismatchInstanceType', {
            type: 'String',
            default: 't3.micro',
            description: 'The instance type for the mismatch cluster',
        });
        this.mismatchInstanceType.overrideLogicalId('MismatchInstanceType');
    }
}
