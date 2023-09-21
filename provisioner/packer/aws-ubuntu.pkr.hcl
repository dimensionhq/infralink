packer {
  required_plugins {
    amazon = {
      version = "~> 1"
      source  = "github.com/hashicorp/amazon"
    }
  }
}

source "amazon-ebs" "ubuntu" {
  ami_name      = "infralink"
  ami_groups    = ["all"]
  instance_type = "t2.micro"
  region        = "us-east-2"
  source_ami_filter {
    filters = {
      #TODO - see here for the list of images
      image-id            = "ami-0e83be366243f524a" #Ubuntu 22.04, us-east-2, amd64
      root-device-type    = "ebs"
      virtualization-type = "hvm"
    }
    most_recent = true
    owners      = ["099720109477"]
  }
  ssh_username = "ubuntu"
}

build {
  name = "infralink"
  sources = [
    "source.amazon-ebs.ubuntu"
  ]
  provisioner "shell" {
    inline = [
      "curl -sSLf https://get.k0s.sh | sudo sh",
      "sudo snap install yq",
    ]
  }
}
