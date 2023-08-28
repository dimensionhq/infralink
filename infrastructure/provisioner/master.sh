#!/usr/bin/env bash

#Install yq
snap install yq

curl -sSLf https://get.k0s.sh | sh
k0s config create > k0s.yaml
#chmod 0644 /etc/k0s/k0s.yaml

#Get the node's external IP address
export IP=$(curl -s ipv4.icanhazip.com)
#Add the IP address to API sans
yq eval -i ".spec.api.sans += \"$IP\"" k0s.yaml

#Install and start the k0s
k0s install controller -c k0s.yaml
k0s start

#Create the token
k0s token create --role=worker --expiry=1h > token-file
