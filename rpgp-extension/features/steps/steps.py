import os
from pathlib import Path

import humanfriendly
from behave import step, then, when

from minifi_test_framework.containers.docker_image_builder import DockerImageBuilder
from minifi_test_framework.core.helpers import wait_for_condition
from minifi_test_framework.core.minifi_test_context import MinifiTestContext
from minifi_test_framework.minifi.controller_service import ControllerService
from minifi_test_framework.minifi.processor import Processor


@when("MiNiFi is started")
def step_impl(context: MinifiTestContext):
    context.get_or_create_default_minifi_container().deploy(context)  # without assert


@step("the built rust extension library is inside minifi's extension folder")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))
    host_path = os.path.normpath(os.path.join(dir_path, "../../../docker_builder/target/libminifi_pgp.so"))
    lib_filename = "libminifi_pgp.so"

    with open(host_path, 'rb') as f:
        lib_content = f.read()

    base_img = context.minifi_container_image
    container_extension_dir = "/opt/minifi/minifi-current/extensions/"

    dockerfile = f"""
FROM {base_img}
COPY --chown=minificpp:minificpp {lib_filename} {container_extension_dir}
RUN chmod 755 {container_extension_dir}{lib_filename}
"""

    builder = DockerImageBuilder(image_tag="apacheminificpp:rusty", dockerfile_content=dockerfile,
        files_on_context={lib_filename: lib_content})

    builder.build()
    context.minifi_container_image = "apacheminificpp:rusty"


@step("an EncryptContentPGP processor with a PGPPublicKeyService is set up")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))

    public_key_service = ControllerService(class_name="PGPPublicKeyService", service_name="my_public_keys")
    alice_public_key = Path(f"{dir_path}/../../test_keys/keyring.asc").read_text()
    public_key_service.add_property("Keyring", alice_public_key)
    context.get_or_create_default_minifi_container().flow_definition.controller_services.append(public_key_service)

    processor = Processor("EncryptContentPGP", "EncryptContentPGP")
    processor.add_property("Public Key Service", "my_public_keys")
    context.get_or_create_default_minifi_container().flow_definition.processors.append(processor)


@step("a DecryptContentPGP processor named DecryptAlice with a PGPPrivateKeyService is set up for Alice")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))

    private_key_service = ControllerService(class_name="PGPPrivateKeyService", service_name="alice_private_key")
    alice_private_key = Path(f"{dir_path}/../../test_keys/alice_private.asc").read_text()
    private_key_service.add_property("Key", alice_private_key)
    private_key_service.add_property("Key Passphrase", "whiterabbit")
    context.get_or_create_default_minifi_container().flow_definition.controller_services.append(private_key_service)

    processor = Processor("DecryptContentPGP", "DecryptAlice")
    processor.add_property("Private Key Service", "alice_private_key")
    context.get_or_create_default_minifi_container().flow_definition.processors.append(processor)


@step("a DecryptContentPGP processor named DecryptBob with a PGPPrivateKeyService is set up for Bob")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))

    private_key_service = ControllerService(class_name="PGPPrivateKeyService", service_name="bob_private_key")
    bob_private_key = Path(f"{dir_path}/../../test_keys/bob_private.asc").read_text()
    private_key_service.add_property("Key", bob_private_key)
    context.get_or_create_default_minifi_container().flow_definition.controller_services.append(private_key_service)

    processor = Processor("DecryptContentPGP", "DecryptBob")
    processor.add_property("Private Key Service", "bob_private_key")
    context.get_or_create_default_minifi_container().flow_definition.processors.append(processor)


@then('Minifi crashes with the following "{crash_msg}" in less than {duration}')
def step_impl(context: MinifiTestContext, crash_msg: str, duration: str):
    duration_seconds = humanfriendly.parse_timespan(duration)
    assert wait_for_condition(
        condition=lambda: context.get_or_create_default_minifi_container().exited and crash_msg in context.get_or_create_default_minifi_container().get_logs(),
        timeout_seconds=duration_seconds, bail_condition=lambda: False, context=context)


@then('an encrypted armored pgp file is placed in the "{directory}" directory in less than {duration}')
def step_impl(context: MinifiTestContext, directory: str, duration: str):
    duration_seconds = humanfriendly.parse_timespan(duration)
    assert wait_for_condition(
        condition=lambda: context.get_or_create_default_minifi_container().directory_contains_file_with_regex(directory,
                                                                                                              "-----BEGIN PGP MESSAGE-----"),
        timeout_seconds=duration_seconds, bail_condition=lambda: False, context=context)
