"""Powerpanel analytics sensor."""
from __future__ import annotations

from homeassistant.components.sensor import (
    SensorDeviceClass,
    SensorEntity,
    SensorStateClass,
)
from homeassistant.const import POWER_WATT
from homeassistant.core import HomeAssistant
from homeassistant.helpers.entity_platform import AddEntitiesCallback
from homeassistant.helpers.typing import ConfigType, DiscoveryInfoType

import requests

def setup_platform(
    hass: HomeAssistant,
    config: ConfigType,
    add_entities: AddEntitiesCallback,
    discovery_info: DiscoveryInfoType | None = None
) -> None:
    add_entities([
      PowerpanelSensor("Load Wattage", POWER_WATT, SensorDeviceClass.POWER, "load_watt")
    ])


class PowerpanelSensor(SensorEntity):
    _attr_state_class = SensorStateClass.MEASUREMENT

    def __init__(self, name, measurement, device_class, param):
      self._attr_name = name
      self._attr_native_unit_of_measurement = measurement
      self._attr_device_class = device_class
      self.param = param

    def update(self) -> None:
      r = requests.get('http://rowlett:3135')
      if r.status_code == 200:
        self._attr_native_value = r.json()[self.param]
