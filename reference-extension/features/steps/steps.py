from behave import then, when
import humanfriendly

from minifi_test_framework.steps import checking_steps        # noqa: F401
from minifi_test_framework.steps import configuration_steps   # noqa: F401
from minifi_test_framework.steps import core_steps            # noqa: F401
from minifi_test_framework.steps import flow_building_steps   # noqa: F401
from minifi_test_framework.core.helpers import wait_for_condition
from minifi_test_framework.core.minifi_test_context import MinifiTestContext

@when("the MiNiFi instance is started without assertions")
def step_impl(context: MinifiTestContext):
    context.get_or_create_default_minifi_container().deploy(context)

@then('Minifi crashes with the following "{crash_msg}" in less than {duration}')
def step_impl(context: MinifiTestContext, crash_msg: str, duration: str):
    duration_seconds = humanfriendly.parse_timespan(duration)
    assert wait_for_condition(condition=lambda: context.get_or_create_default_minifi_container().exited and crash_msg in context.get_or_create_default_minifi_container().get_logs(),
                              timeout_seconds=duration_seconds, bail_condition=lambda: False, context=context)
