import React, { useState } from 'react';
import axios from 'axios';
import {
  EuiButton,
  EuiFieldText,
  EuiFlexItem,
  EuiFormRow,
  EuiPanel,
  EuiPopover,
  EuiSpacer,
  EuiText,
} from '@elastic/eui';

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

  const [isRadioPopOverOpen, setIsRadioPopOverOpen] = useState<boolean>(false);
  const radioButton = <EuiButton onClick={() => setIsRadioPopOverOpen(true)}>Transmit</EuiButton>;

  return (
    <EuiFlexItem>
      <EuiSpacer />
      <EuiPanel style={{ maxWidth: 300 }}>
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
        <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
          <EuiPopover
            id="trapFocus"
            ownFocus
            button={radioButton}
            isOpen={isRadioPopOverOpen}
            closePopover={() => {
              setIsRadioPopOverOpen(false);
              setRadioStatus({
                isInProgress: false,
                bytesString: '',
                isValid: false,
                response: null,
              });
            }}
          >
            <EuiFormRow
              style={{ minWidth: 300 }}
              label="Bytes sequence to send"
              helpText={radioStatus.response ? `Response: [${radioStatus.response.join(', ')}]` : ''}
              isInvalid={showError}
              error={['Bytes should be a comma separated list of `u8` values.']}
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
            <EuiSpacer />
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
          </EuiPopover>
        </EuiFormRow>
      </EuiPanel>
    </EuiFlexItem>
  );
}
