import React from 'react';
import { EuiFlexItem, EuiFormRow, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';

export function AlarmsSection() {
  const content = (
    <EuiPanel>
      <EuiFormRow label="Hours" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Minutes" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Seconds" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{'Unknown'}</EuiText>
      </EuiFormRow>
    </EuiPanel>
  );

  return (
    <EuiFlexItem>
      <EuiSpacer />
      {content}
    </EuiFlexItem>
  );
}
