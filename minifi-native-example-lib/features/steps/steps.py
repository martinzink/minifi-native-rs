from behave import step
import os
import logging
import time
import humanfriendly

from minifi_test_framework.steps import checking_steps
from minifi_test_framework.steps import configuration_steps
from minifi_test_framework.steps import core_steps
from minifi_test_framework.steps import flow_building_steps
from minifi_test_framework.core.minifi_test_context import MinifiTestContext
from minifi_test_framework.minifi.processor import Processor
from minifi_test_framework.containers.host_file import HostFile

@step("the built rust extension library is inside minifi's extension folder")
def step_impl(context: MinifiTestContext):
    dir_path = os.path.dirname(os.path.realpath(__file__))
    host_path = f"{dir_path}/../../so_to_test/libminifi_native_example_lib.so"
    context.minifi_container.host_files.append(HostFile(f"{dir_path}/../../so_to_test/libminifi_native_example_lib.so", "/opt/minifi/minifi-current/extensions/libminifi-rust.so"))

@then("Waits for {duration}")
def step_impl(context: MinifiTestContext, duration: str):
    timeout_in_seconds = humanfriendly.parse_timespan(duration)
    time.sleep(timeout_in_seconds)