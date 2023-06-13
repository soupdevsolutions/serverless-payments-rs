terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.4.0"
    }
  }

  backend "s3" {
    bucket = "serveless-payments-tf-state"
    key    = "terraform.tfstate"
    region = "eu-west-1"
  }

  required_version = "~> 1.0"
}
