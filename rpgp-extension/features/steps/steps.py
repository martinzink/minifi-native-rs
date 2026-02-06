from behave import step, then, when
import os
import time
import humanfriendly
from pathlib import Path

from minifi_test_framework.core.helpers import wait_for_condition
from minifi_test_framework.minifi.controller_service import ControllerService
from minifi_test_framework.minifi.processor import Processor
from minifi_test_framework.steps import checking_steps
from minifi_test_framework.steps import configuration_steps
from minifi_test_framework.steps import core_steps
from minifi_test_framework.steps import flow_building_steps
from minifi_test_framework.core.minifi_test_context import MinifiTestContext
from minifi_test_framework.containers.host_file import HostFile

@when("MiNiFi is started")
def step_impl(context: MinifiTestContext):
    context.get_or_create_default_minifi_container().deploy(context)
    # without assert

@step("the built rust extension library is inside minifi's extension folder")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))
    host_path = f"{dir_path}/../../../docker_builder/target/librpgp_extension.so"
    print(host_path)
    context.get_or_create_default_minifi_container().host_files.append(HostFile("/opt/minifi/minifi-current/extensions/libminifi-pgp.so", host_path))

@step("an EncryptContentPGP processor with a PgpPublicKeyService is set up")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))

    public_key_service = ControllerService(class_name="PgpPublicKeyService", service_name="my_public_keys")
    alice_public_key = Path(f"{dir_path}/../../test_keys/alice.asc").read_text()
    public_key_service.add_property("Keyring", alice_public_key)
    context.get_or_create_default_minifi_container().flow_definition.controller_services.append(public_key_service)

    processor = Processor("EncryptContentPGP", "EncryptContentPGP")
    processor.add_property("Public Key Service", "my_public_keys")
    processor.add_property("Public Key Search", "Alice")
    context.get_or_create_default_minifi_container().flow_definition.processors.append(processor)


@step("a DecryptContentPGP processor with a PgpPrivateKeyService is set up")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))

    private_key_service = ControllerService(class_name="PgpPrivateKeyService", service_name="my_private_key")
    alice_private_key = Path(f"{dir_path}/../../test_keys/alice_private.asc").read_text()
    private_key_service.add_property("Key", alice_private_key)
    private_key_service.add_property("Key Passphrase", "123")
    context.get_or_create_default_minifi_container().flow_definition.controller_services.append(private_key_service)

    processor = Processor("DecryptContentPGP", "DecryptContentPGP")
    processor.add_property("Private Key Service", "my_private_key")
    context.get_or_create_default_minifi_container().flow_definition.processors.append(processor)
@then('Minifi crashes with the following "{crash_msg}" in less than {duration}')
def step_impl(context: MinifiTestContext, crash_msg: str, duration: str):
    duration_seconds = humanfriendly.parse_timespan(duration)
    assert wait_for_condition(condition=lambda: context.get_or_create_default_minifi_container().exited and crash_msg in context.get_or_create_default_minifi_container().get_logs(),
                              timeout_seconds=duration_seconds, bail_condition=lambda: False, context=context)
