#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { MismatchStack } from '../lib/mismatch-stack';
import { MismatchEcrStack } from '../lib/mismatch-ecr-stack';

const app = new cdk.App();

new MismatchEcrStack(app, 'MismatchEcrStack', {
    stackName: app.node.tryGetContext('stack-name'),
    env: {
        account: process.env.CDK_DEFAULT_ACCOUNT,
        region: process.env.CDK_DEFAULT_REGION,
    },
});

new MismatchStack(app, 'MismatchStack', {
    stackName: app.node.tryGetContext('stack-name'),
    env: {
        account: process.env.CDK_DEFAULT_ACCOUNT,
        region: process.env.CDK_DEFAULT_REGION,
    },
});
