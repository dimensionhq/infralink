resource "aws_default_vpc" "main" {}

data "aws_security_group" "main" {
  name = "default"
}

data "aws_subnets" "main" {
  filter {
    name   = "vpc-id"
    values = [aws_default_vpc.main.id]
  }
}

module "master" {
  source = "terraform-aws-modules/ec2-instance/aws"

  name                   = "master"
  ami                    = "ami-0b5801d081fa3a76c"
  instance_type          = "t4g.micro"
  key_name               = "martins.eglitis"
  monitoring             = false
  vpc_security_group_ids = [data.aws_security_group.main.id]
  subnet_id              = data.aws_subnets.main.ids[0]

  tags = {
    Terraform = "true"
  }
}

module "worker" {
  source = "terraform-aws-modules/ec2-instance/aws"

  name                   = "worker"
  ami                    = "ami-0b5801d081fa3a76c"
  instance_type          = "t4g.micro"
  key_name               = "martins.eglitis"
  monitoring             = false
  vpc_security_group_ids = [data.aws_security_group.main.id]
  subnet_id              = data.aws_subnets.main.ids[0]

  tags = {
    Terraform = "true"
  }
}