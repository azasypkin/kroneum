import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { EuiFlexItem, EuiFormRow, EuiLoadingContent, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';

interface DeviceID {
  bus: number;
  address: number;
  vendorID: number;
  productID: number;
  manufacturer: string;
}

export function InfoSection() {
  const [id, setID] = useState<DeviceID | null>(null);
  useEffect(() => {
    axios.get('/api/id').then(({ data }) => setID(data));
  }, []);

  const content = id ? (
    <EuiPanel>
      <EuiFormRow label="Bus" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{id?.bus ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Address" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{id?.address ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Vendor ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{id ? `0x${id.vendorID.toString(16).toUpperCase()}` : 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Product ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{id ? `0x${id.productID.toString(16).toUpperCase()}` : 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Manufacturer" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{id?.manufacturer ?? 'Unknown'}</EuiText>
      </EuiFormRow>
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
