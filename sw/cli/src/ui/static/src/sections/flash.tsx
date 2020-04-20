import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { EuiFlexItem, EuiFormRow, EuiLoadingContent, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';

export function FlashSection() {
  const [slots, setSlots] = useState<number[] | null>(null);
  useEffect(() => {
    axios.get('/api/flash').then(({ data }) => setSlots(data));
  }, []);

  const content = slots ? (
    <EuiPanel>
      {slots.map((slotContent, slotIndex) => {
        return (
          <EuiFormRow
            key={slotIndex}
            label={slotIndex === 0 ? 'Configuration Slot' : `Custom Slot#${slotIndex}`}
            display="columnCompressed"
            style={{ alignItems: 'center' }}
          >
            <EuiText size="s">{slotContent}</EuiText>
          </EuiFormRow>
        );
      })}
    </EuiPanel>
  ) : (
    <EuiPanel>
      <EuiLoadingContent lines={5} />
    </EuiPanel>
  );

  return (
    <EuiFlexItem>
      <EuiSpacer />
      {content}
    </EuiFlexItem>
  );
}
