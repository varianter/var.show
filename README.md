# var.show

Work in progress "deploy yourself" URL shortner written in Rust for running on Azure Functions.

## Features initial release

- POST URL and return random shorted
- POST URL with shorted string specified (named)
- Redirect `var.show/<ID>` to full URL

## Features next release

- PUT URL (update)
- Administration

## Deploy

[![Deploy to Azure](https://aka.ms/deploytoazurebutton)](https://portal.azure.com/#create/Microsoft.Template/uri/https%3A%2F%2Fraw.githubusercontent.com%2Fvarianter%2Fvar.show%2Fmaster%2Fazuredeploy.json)

Press the above button.

Or, do it manually with the Azure CLI and the provided `azuredeploy.json` ARM-file:

```bash
# create resource group for the resources
az group create --location westeurope --name varshow

# create the resources and passing the appName
az group deployment create -g varshow --template-file azuredeploy.json --parameters appName=varshow
```

In this example we name both the resource group and the function app `varshow`.

Other parameters you can pass when creating the resources are:

- `serverFarmId` (override if you want to use a pre-existing App Service plan)
- `dockerImage` (default is `varianter\varshow:latest`, override if you want to use another image)
