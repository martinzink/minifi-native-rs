from behave import step

from minifi_test_framework.steps import checking_steps
from minifi_test_framework.steps import configuration_steps
from minifi_test_framework.steps import core_steps
from minifi_test_framework.steps import flow_building_steps
from minifi_test_framework.core.minifi_test_context import MinifiTestContext
from minifi_test_framework.minifi.processor import Processor
from minifi_test_framework.containers.host_file import HostFile

@step("the built rust extension library is inside minifi's extension folder")
def step_impl(context: MinifiTestContext):
    context.minifi_container.host_files.append(HostFile("/Users/mzink/Downloads/test_c_api/libminifi_native_example_lib.so", "/opt/minifi/minifi-current/extensions/libminifi-rust.so"))

