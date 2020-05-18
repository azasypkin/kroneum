import React, { useEffect, useState } from 'react';
import {
  EuiButton,
  EuiFieldText,
  EuiFlexItem,
  EuiFormRow,
  EuiLink,
  EuiLoadingContent,
  EuiPanel,
  EuiSpacer,
  EuiText,
} from '@elastic/eui';
import axios from 'axios';

export function AlarmsSection() {
  const [alarm, setAlarm] = useState<string | null>(null);
  useEffect(() => {
    axios.get('/api/alarm').then(({ data }) => setAlarm(data));
  }, []);

  const [newAlarm, setNewAlarm] = useState<string>('');
  const [isSettingNewAlarm, setIsSettingNewAlarm] = useState<boolean>(false);

  const content = alarm ? (
    <EuiPanel>
      <EuiFormRow label="Current alarm" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{alarm}</EuiText>
      </EuiFormRow>
      <EuiSpacer />
      <EuiFormRow
        style={{ alignItems: 'center' }}
        display="columnCompressed"
        label="Set alarm"
        helpText={
          <EuiText size="xs">
            See the alarm format{' '}
            <EuiLink href="https://docs.rs/humantime/2.0.0/humantime/struct.Duration.html" target="_blank">
              here
            </EuiLink>
            .
          </EuiText>
        }
      >
        <>
          <EuiFieldText
            placeholder="Enter new alarm."
            value={newAlarm}
            onChange={(ev) => {
              setNewAlarm(ev.target.value.trim());
            }}
            append={
              <EuiButton
                isDisabled={isSettingNewAlarm}
                isLoading={isSettingNewAlarm}
                fill
                onClick={() => {
                  setIsSettingNewAlarm(true);
                  axios.post('/api/alarm/set', { alarm: newAlarm }).then(
                    () => setIsSettingNewAlarm(false),
                    () => setIsSettingNewAlarm(false),
                  );
                }}
              >
                Set
              </EuiButton>
            }
          />
        </>
      </EuiFormRow>
    </EuiPanel>
  ) : (
    <EuiPanel>
      <EuiLoadingContent lines={1} />
    </EuiPanel>
  );

  return (
    <EuiFlexItem>
      <EuiSpacer />
      {content}
    </EuiFlexItem>
  );
}
