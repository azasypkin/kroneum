import React, { useState } from 'react';
import axios from 'axios';
import { EuiButton, EuiFieldText, EuiFlexItem, EuiFormRow, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';
import { Note, Player } from '../audio';

interface EchoStatus {
  isInProgress: boolean;
  bytesString: string;
  isValid: boolean;
  response: number[] | null;
}

const MELODY: Array<[Note, number]> = [
  [Note.A5, 0.25],
  [Note.ASharp5, 0.25],
  [Note.B5, 0.25],
  [Note.C6, 0.25],
  [Note.CSharp6, 0.25],
  [Note.D6, 0.25],
  [Note.DSharp6, 0.25],
  [Note.E6, 0.25],
  [Note.F6, 0.25],
  [Note.FSharp6, 0.25],
  [Note.G6, 0.25],
  [Note.GSharp6, 0.25],
  [Note.A6, 0.25],
];

export function DiagnosticsSection() {
  const [echoStatus, setEchoStatus] = useState<EchoStatus>({
    isInProgress: false,
    bytesString: '',
    isValid: false,
    response: null,
  });

  const showError = !echoStatus.isValid && echoStatus.bytesString.length > 0;
  return (
    <>
      <EuiFlexItem>
        <EuiSpacer />
        <EuiPanel>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiButton onClick={() => axios.get('/api/beep')}>Send beep</EuiButton>
          </EuiFormRow>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiButton
              onClick={async () => {
                Player.play(MELODY);

                await axios.post(
                  '/api/play',
                  MELODY.map(([note, duration]) => [note, duration * 400]),
                );
              }}
            >
              Play melody
            </EuiButton>
          </EuiFormRow>
        </EuiPanel>
      </EuiFlexItem>
      <EuiFlexItem>
        <EuiSpacer />
        <EuiPanel>
          <EuiFormRow
            style={{ alignItems: 'center' }}
            display="columnCompressed"
            label="Bytes to send"
            isInvalid={showError}
            error={['Should be a comma separated list of `u8` values.']}
          >
            <EuiFieldText
              placeholder="Enter comma separated `u8` numbers..."
              value={echoStatus.bytesString}
              name="text"
              isInvalid={showError}
              onChange={(ev) => {
                setEchoStatus({
                  ...echoStatus,
                  response: null,
                  isValid:
                    ev.target.value &&
                    ev.target.value.split(',').every((value) => {
                      const intValue = parseInt(value.trim());
                      return Number.isInteger(intValue) && intValue >= 0 && intValue < 256;
                    }),
                  bytesString: ev.target.value,
                });
              }}
            />
          </EuiFormRow>
          <EuiFormRow label="Response" display="columnCompressed">
            <EuiText>{echoStatus.response ? `[${echoStatus.response.join(', ')}]` : 'n/a'}</EuiText>
          </EuiFormRow>
          <EuiFormRow>
            <EuiButton
              isDisabled={!echoStatus.isValid}
              isLoading={echoStatus.isInProgress}
              fill
              onClick={() => {
                setEchoStatus({
                  ...echoStatus,
                  isInProgress: true,
                });

                axios
                  .post(
                    '/api/echo',
                    echoStatus.bytesString.split(',').map((value) => parseInt(value.trim())),
                  )
                  .then(({ data }) => {
                    setEchoStatus({
                      ...echoStatus,
                      isInProgress: false,
                      response: data,
                    });
                  });
              }}
            >
              Send echo
            </EuiButton>
          </EuiFormRow>
        </EuiPanel>
      </EuiFlexItem>
    </>
  );
}
