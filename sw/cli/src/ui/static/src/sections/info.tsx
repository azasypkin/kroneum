import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { EuiFlexItem, EuiFormRow, EuiLoadingContent, EuiPanel, EuiSpacer, EuiText } from '@elastic/eui';

interface Info {
  device: {
    bus: number;
    address: number;
    vendorID: number;
    productID: number;
    manufacturer: string;
  };
  system: { id: string; flashSizeKb: number };
}

export function InfoSection() {
  const [info, setInfo] = useState<Info | null>(null);
  useEffect(() => {
    axios.get('/api/info').then(({ data }) => setInfo(data));
  }, []);

  const content = info ? (
    <EuiPanel>
      <EuiFormRow label="Bus" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{info.device.bus ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Address" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{info.device.address ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Vendor ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{info ? `0x${info.device.vendorID.toString(16).toUpperCase()}` : 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Product ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{info ? `0x${info.device.productID.toString(16).toUpperCase()}` : 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Manufacturer" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{info.device.manufacturer ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="System ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{info.system.id ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Flash Size" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{`${info.system.flashSizeKb} KB` ?? 'Unknown'}</EuiText>
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
