listener "tcp" {
    address = "0.0.0.0:8200"
    cluster_address = "0.0.0.0:8201"
    tls_disable = true
}
ui = true
storage "consul" {
    address = "XXXXXXX:8500"
    path = "vault"
    token = "XXXXXXXX"
}
service_registration "consul" {
    address = "XXXXXXXX:8500"
    token = "XXXXXXXX"
    service = "vault"
}
api_addr =  "http://XXXXXXX:8200"
cluster_addr = "http://XXXXXX:8201"
