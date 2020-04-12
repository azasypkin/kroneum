import React, { useState } from 'react';
import axios from 'axios';
import {
  EuiButton,
  EuiFlexItem,
  EuiFormRow,
  EuiText,
  EuiLink,
  EuiPanel,
  EuiSpacer,
  EuiFieldNumber,
} from '@elastic/eui';

export function KeyboardSection() {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [keyCode, setKeyCode] = useState<number>(57);
  const [delay, setDelay] = useState<number>(1);

  const isInputValid = keyCode >= 0 && keyCode <= 255 && delay >= 0 && delay <= 2;

  return (
    <EuiFlexItem>
      <EuiSpacer />
      <EuiPanel>
        <EuiFormRow
          style={{ alignItems: 'center' }}
          display="columnCompressed"
          label="Key code"
          helpText={
            <EuiText size="xs">
              See the list of key codes{' '}
              <EuiLink href="https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf" target="_blank">
                here
              </EuiLink>
              .
            </EuiText>
          }
        >
          <EuiFieldNumber
            placeholder="Enter u8 key code."
            value={keyCode}
            min={0}
            max={255}
            onChange={(ev) => {
              setKeyCode(parseInt(ev.target.value.trim()) || 0);
            }}
          />
        </EuiFormRow>
        <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed" label="Delay">
          <EuiFieldNumber
            placeholder="Enter delay in seconds."
            value={delay}
            min={0}
            max={2}
            onChange={(ev) => {
              setDelay(parseInt(ev.target.value.trim()) || 0);
            }}
          />
        </EuiFormRow>
        <EuiFormRow display="columnCompressed" style={{ alignItems: 'center' }}>
          <EuiButton
            isDisabled={isLoading || !isInputValid}
            isLoading={isLoading}
            fill
            onClick={() => {
              setIsLoading(true);
              axios.post('/api/key', [keyCode, delay]).then(() => setIsLoading(false));
            }}
          >
            Send
          </EuiButton>
        </EuiFormRow>
      </EuiPanel>
    </EuiFlexItem>
  );
}
