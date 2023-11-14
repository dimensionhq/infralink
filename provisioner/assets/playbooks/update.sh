#!/bin/bash

chmod 0600 /home/martins/.infralink/kubeconfig

helm --kubeconfig /home/martins/.infralink/kubeconfig upgrade \
 --install aws-cloud-controller-manager aws-cloud-controller-manager/aws-cloud-controller-manager \
 --values values.yaml