terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.4.0"
    }
    stripe = {
      source  = "franckverrot/stripe"
      version = "1.8.0"
    }
  }

  backend "s3" {
    bucket = "serveless-payments-tf-state"
    key    = "terraform.tfstate"
    region = "eu-west-1"
  }

  required_version = "~> 0.14"
}

provider "aws" {
  region     = var.AWS_REGION
  access_key = var.AWS_ACCESS_KEY_ID
  secret_key = var.AWS_SECRET_ACCESS_KEY
}

provider "stripe" {
  api_token = var.STRIPE_API_KEY
}
