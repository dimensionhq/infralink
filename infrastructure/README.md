# AWS

IMO in this case, we really, really have to give the user some pre-defined setup. Why? Because otherwise we will be basically just copy-pasting whatever functionality the CP has. 

So the setup might be (for AWS):

- ECR registry
- 1 s3 bucket for storing the Pulumi state
- 1 new VPC
- New subnets, based on the number of AZs available (1 subnet per AZ)
- 2xEC2 (at least t4g.micro)