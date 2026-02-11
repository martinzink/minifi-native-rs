from behave import step, then, when
import os
import time
import humanfriendly

from minifi_test_framework.core.helpers import wait_for_condition
from minifi_test_framework.steps import checking_steps
from minifi_test_framework.steps import configuration_steps
from minifi_test_framework.steps import core_steps
from minifi_test_framework.steps import flow_building_steps
from minifi_test_framework.core.minifi_test_context import MinifiTestContext
from minifi_test_framework.containers.host_file import HostFile


@step("the built rust extension library is inside minifi's extension folder")
def step_impl(context: MinifiTestContext):
    if os.name != 'nt':
        dir_path = os.path.dirname(os.path.realpath(__file__))
        host_path = f"{dir_path}/../../../docker_builder/target/librust_reference_extension.so"
        context.get_or_create_default_minifi_container().host_files.append(HostFile("/opt/minifi/minifi-current/extensions/libminifi-rust.so", host_path))

@then('Minifi crashes with the following "{crash_msg}" in less than {duration}')
def step_impl(context: MinifiTestContext, crash_msg: str, duration: str):
    duration_seconds = humanfriendly.parse_timespan(duration)
    assert wait_for_condition(condition=lambda: context.get_or_create_default_minifi_container().exited and crash_msg in context.get_or_create_default_minifi_container().get_logs(),
                              timeout_seconds=duration_seconds, bail_condition=lambda: False, context=context)
