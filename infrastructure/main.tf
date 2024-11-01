# Create a resource group
resource "azurerm_resource_group" "rg" {
  name     = "my-aks-rg"
  location = "eastus"
}

# Create a Kubernetes cluster
resource "azurerm_kubernetes_cluster" "aks" {
  name                = "my-aks-cluster"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
  dns_prefix          = "my-aks-cluster"

  default_node_pool {
    name       = "default"
    node_count = 1
    vm_size    = "Standard_B2s" # The cheapest node size available
  }

  identity {
    type = "SystemAssigned"
  }
}

# Output the cluster name and kubeconfig
output "cluster_name" {
  value = azurerm_kubernetes_cluster.aks.name
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.aks.kube_config_raw
  sensitive = true
}
