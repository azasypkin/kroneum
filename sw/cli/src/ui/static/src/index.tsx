import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';
import {
  EuiButton,
  EuiPage,
  EuiPageBody,
  EuiPageHeader,
  EuiPageHeaderSection,
  EuiTitle,
  EuiPageContent,
  EuiPageContentHeader,
  EuiPageContentHeaderSection,
  EuiPageContentBody,
  EuiPanel,
  EuiFormRow,
  EuiSpacer,
  EuiText,
  EuiFlexGroup,
  EuiFlexItem,
} from '@elastic/eui';
import axios from 'axios';

interface DeviceInfo {
  identifier: {
    bus: number;
    address: number;
    vendorID: number;
    productID: number;
    manufacturer: string;
  };
  flash: number[];
}

const IndexPage = () => {
  const [info, setInfo] = useState<DeviceInfo | null>(null);
  useEffect(() => {
    axios.get('/api/info').then(({ data }) => setInfo(data));
  }, []);

  return (
    <EuiPage>
      <EuiPageBody>
        <EuiPageHeader>
          <EuiPageHeaderSection>
            <EuiTitle size="l">
              <h1>Kroneum</h1>
            </EuiTitle>
          </EuiPageHeaderSection>
        </EuiPageHeader>
        <EuiPageContent>
          <EuiFlexGroup>
            <EuiFlexItem>
              <EuiPageContentHeader>
                <EuiPageContentHeaderSection>
                  <EuiTitle>
                    <h2>Device information</h2>
                  </EuiTitle>
                </EuiPageContentHeaderSection>
              </EuiPageContentHeader>
              <EuiPageContentBody>
                <EuiPanel style={{ maxWidth: 300 }}>
                  <EuiFormRow label="Bus" display="columnCompressed" style={{ alignItems: 'center' }}>
                    <EuiText size="s">{info?.identifier.bus ?? 'Unknown'}</EuiText>
                  </EuiFormRow>
                  <EuiFormRow label="Address" display="columnCompressed" style={{ alignItems: 'center' }}>
                    <EuiText size="s">{info?.identifier.address ?? 'Unknown'}</EuiText>
                  </EuiFormRow>
                  <EuiFormRow label="Vendor ID" display="columnCompressed" style={{ alignItems: 'center' }}>
                    <EuiText size="s">
                      {info ? `0x${info.identifier.vendorID.toString(16).toUpperCase()}` : 'Unknown'}
                    </EuiText>
                  </EuiFormRow>
                  <EuiFormRow label="Product ID" display="columnCompressed" style={{ alignItems: 'center' }}>
                    <EuiText size="s">
                      {info ? `0x${info.identifier.productID.toString(16).toUpperCase()}` : 'Unknown'}
                    </EuiText>
                  </EuiFormRow>
                  <EuiFormRow label="Manufacturer" display="columnCompressed" style={{ alignItems: 'center' }}>
                    <EuiText size="s">{info?.identifier.manufacturer ?? 'Unknown'}</EuiText>
                  </EuiFormRow>
                </EuiPanel>
              </EuiPageContentBody>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiPageContentHeader>
                <EuiPageContentHeaderSection>
                  <EuiTitle>
                    <h2>Current Alarm</h2>
                  </EuiTitle>
                </EuiPageContentHeaderSection>
              </EuiPageContentHeader>
              <EuiPageContentBody>
                <EuiPanel style={{ maxWidth: 300 }}>
                  <EuiFormRow label="Seconds" display="columnCompressed" style={{ alignItems: 'center' }}>
                    <EuiText size="s">{info?.identifier.bus ?? 'Unknown'}</EuiText>
                  </EuiFormRow>
                </EuiPanel>
              </EuiPageContentBody>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiPageContentHeader>
                <EuiPageContentHeaderSection>
                  <EuiTitle>
                    <h2>Flash Slots</h2>
                  </EuiTitle>
                </EuiPageContentHeaderSection>
              </EuiPageContentHeader>
              <EuiPageContentBody>
                <EuiPanel style={{ maxWidth: 300 }}>
                  {(info?.flash ?? []).map((slotContent, slotIndex) => {
                    return (
                      <EuiFormRow
                        key={slotIndex}
                        label={`Slot#${slotIndex + 1}`}
                        display="columnCompressed"
                        style={{ alignItems: 'center' }}
                      >
                        <EuiText size="s">{slotContent}</EuiText>
                      </EuiFormRow>
                    );
                  })}
                </EuiPanel>
              </EuiPageContentBody>
            </EuiFlexItem>
          </EuiFlexGroup>
          <EuiSpacer size="xl" />
          <EuiPageContentHeader>
            <EuiPageContentHeaderSection>
              <EuiTitle>
                <h2>Device actions</h2>
              </EuiTitle>
            </EuiPageContentHeaderSection>
          </EuiPageContentHeader>
          <EuiPageContentBody>
            <EuiPanel style={{ maxWidth: 300 }}>
              <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
                <EuiButton onClick={() => axios.get('/api/beep')}>Beep</EuiButton>
              </EuiFormRow>
            </EuiPanel>
          </EuiPageContentBody>
        </EuiPageContent>
      </EuiPageBody>
    </EuiPage>
  );
};

ReactDOM.render(<IndexPage />, document.getElementById('root'));
