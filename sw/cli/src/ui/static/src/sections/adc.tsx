import React, { useState } from 'react';
import axios from 'axios';
import { EuiButton, EuiFlexItem, EuiFormRow, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';

interface ADCStatus {
  isInProgress: boolean;
  response: number | null;
}

/*  function reload() {
  setADCStatus({
    ...adcStatus,
    isInProgress: true,
  });

  return axios
    .get('/api/adc/1')
    .then(
      ({ data }) => data,
      () => 'Error',
    )
    .then(data => {
      setADCStatus({
        isInProgress: false,
        response: data,
      });
      setTimeout(() => reload(), 1000);
    });
}

useEffect(() => {
  setTimeout(() => reload(), 1000);
}, []);*/

export function ADCSection() {
  const [adcStatus, setADCStatus] = useState<ADCStatus>({
    isInProgress: false,
    response: null,
  });

  return (
    <EuiFlexItem>
      <EuiSpacer />
      <EuiPanel>
        <EuiFormRow label="Ch#1 value" display="columnCompressed" style={{ alignItems: 'center' }}>
          <EuiText size="s">{adcStatus.response ?? 'Unknown'}</EuiText>
        </EuiFormRow>
        <EuiFormRow display="columnCompressed" style={{ alignItems: 'center' }}>
          <EuiButton
            isLoading={adcStatus.isInProgress}
            fill
            onClick={() => {
              setADCStatus({
                ...adcStatus,
                isInProgress: true,
              });

              axios.get('/api/adc/1').then(({ data }) => {
                setADCStatus({
                  isInProgress: false,
                  response: data,
                });
              });
            }}
          >
            Read
          </EuiButton>
        </EuiFormRow>
      </EuiPanel>
    </EuiFlexItem>
  );
}
