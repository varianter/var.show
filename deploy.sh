# create resource group with app name
az group create --location $2 --name $1 --query "properties.provisioningState"

# create resources
az group deployment create -g $1 --template-file azuredeploy.json --parameters appName=$1 location=$2 serverFarmId=$3 --query "properties.provisioningState"