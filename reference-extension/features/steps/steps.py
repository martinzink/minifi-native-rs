from behave import step, then, when
import os
import time
import humanfriendly

from minifi_test_framework.containers.docker_image_builder import DockerImageBuilder
from minifi_test_framework.core.helpers import wait_for_condition
from minifi_test_framework.steps import checking_steps
from minifi_test_framework.steps import configuration_steps
from minifi_test_framework.steps import core_steps
from minifi_test_framework.steps import flow_building_steps
from minifi_test_framework.core.minifi_test_context import MinifiTestContext

@when("the MiNiFi instance is started without assertions")
def step_impl(context: MinifiTestContext):
    context.get_or_create_default_minifi_container().deploy(context)

@then('Minifi crashes with the following "{crash_msg}" in less than {duration}')
def step_impl(context: MinifiTestContext, crash_msg: str, duration: str):
    duration_seconds = humanfriendly.parse_timespan(duration)
    assert wait_for_condition(condition=lambda: context.get_or_create_default_minifi_container().exited and crash_msg in context.get_or_create_default_minifi_container().get_logs(),
                              timeout_seconds=duration_seconds, bail_condition=lambda: False, context=context)
