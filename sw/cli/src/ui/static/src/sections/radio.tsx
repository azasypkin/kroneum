import React, { useState } from 'react';
import axios from 'axios';
import { EuiButton, EuiFieldText, EuiFlexItem, EuiFormRow, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';

interface RadioStatus {
  isInProgress: boolean;
  bytesString: string;
  isValid: boolean;
  response: number[] | null;
}

export function RadioSection() {
  const [radioStatus, setRadioStatus] = useState<RadioStatus>({
    isInProgress: false,
    bytesString: '',
    isValid: false,
    response: null,
  });

  const showError = !radioStatus.isValid && radioStatus.bytesString.length > 0;
  return (
    <>
      <EuiFlexItem>
        <EuiSpacer />
        <EuiPanel>
          <EuiFormRow label="Last response" display="columnCompressed" style={{ alignItems: 'center' }}>
            <EuiText size="s">
              {Array.isArray(radioStatus.response) ? `[${radioStatus.response.join(', ')}]` : 'Unknown'}
            </EuiText>
          </EuiFormRow>
          <EuiFormRow display="columnCompressed" style={{ alignItems: 'center' }}>
            <EuiButton
              isLoading={radioStatus.isInProgress}
              fill
              onClick={() => {
                setRadioStatus({
                  ...radioStatus,
                  isInProgress: true,
                });

                axios.get('/api/radio/receive').then(({ data }) => {
                  setRadioStatus({
                    ...radioStatus,
                    isInProgress: false,
                    response: data,
                  });
                });
              }}
            >
              Receive
            </EuiButton>
          </EuiFormRow>
          <EuiFormRow display="columnCompressed" style={{ alignItems: 'center' }}>
            <EuiButton
              isLoading={radioStatus.isInProgress}
              fill
              onClick={() => {
                setRadioStatus({
                  ...radioStatus,
                  isInProgress: true,
                });

                axios.get('/api/radio/status').then(({ data }) => {
                  setRadioStatus({
                    ...radioStatus,
                    isInProgress: false,
                    response: data,
                  });
                });
              }}
            >
              Status
            </EuiButton>
          </EuiFormRow>
        </EuiPanel>
      </EuiFlexItem>
      <EuiFlexItem>
        <EuiSpacer />
        <EuiPanel>
          <EuiFormRow
            display="columnCompressed"
            label="Bytes to send"
            isInvalid={showError}
            error={['Should be a comma separated list of `u8` values.']}
          >
            <EuiFieldText
              placeholder="Enter comma separated `u8` numbers..."
              value={radioStatus.bytesString}
              name="text"
              isInvalid={showError}
              onChange={(ev) =>
                setRadioStatus({
                  ...radioStatus,
                  response: null,
                  isValid:
                    ev.target.value &&
                    ev.target.value.split(',').every((value) => {
                      const intValue = parseInt(value.trim());
                      return Number.isInteger(intValue) && intValue >= 0 && intValue < 256;
                    }),
                  bytesString: ev.target.value,
                })
              }
            />
          </EuiFormRow>
          <EuiFormRow label="Response" display="columnCompressed">
            <EuiText>{radioStatus.response ? `[${radioStatus.response.join(', ')}]` : 'n/a'}</EuiText>
          </EuiFormRow>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiButton
              isDisabled={!radioStatus.isValid}
              isLoading={radioStatus.isInProgress}
              fill
              onClick={() => {
                setRadioStatus({
                  ...radioStatus,
                  isInProgress: true,
                });

                axios
                  .post(
                    '/api/radio/transmit',
                    radioStatus.bytesString.split(',').map((value) => parseInt(value.trim())),
                  )
                  .then(({ data }) => {
                    setRadioStatus({
                      ...radioStatus,
                      isInProgress: false,
                      response: data,
                    });
                  });
              }}
            >
              Transmit
            </EuiButton>
          </EuiFormRow>
        </EuiPanel>
      </EuiFlexItem>
    </>
  );
}
