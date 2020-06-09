soralet_id ?= lte-m-button
soracom_profile ?= default
output = target/wasm32-unknown-unknown/release/lte_m_button.wasm

.PHONY: create-soralet
create-soralet:
	soracom --profile $(soracom_profile) soralets create --soralet-id $(soralet_id)

.PHONY: upload
upload:
	soracom --profile $(soracom_profile) soralets upload --soralet-id $(soralet_id) --content-type application/octet-stream --body @$(output)

payload = '{"batteryLevel":1,"clickType":1,"clickTypeName":"SINGLE"}'

.PHONY: test
test:
	soracom --profile $(soracom_profile) soralets exec --soralet-id $(soralet_id) --version "\$$LATEST" --direction uplink --content-type "application/json" --payload $(payload) --body @test-uplink.json

.PHONY: log
log:
	soracom --profile $(soracom_profile) soralets get-logs --soralet-id $(soralet_id)
