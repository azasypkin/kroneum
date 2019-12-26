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
  EuiPopover,
  EuiComboBox,
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

  const [echo, setEcho] = useState<number[]>([0]);
  const [isEchoPopOverOpen, setIsEchoPopOverOpen] = useState<boolean>(false);

  const echoButton = <EuiButton onClick={() => setIsEchoPopOverOpen(true)}>Send Echo</EuiButton>;

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
                    <h2>Current alarm</h2>
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
                    <h2>Flash slots</h2>
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
              <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
                <EuiButton onClick={() => axios.get('/api/melody')}>Play melody</EuiButton>
              </EuiFormRow>
              <EuiSpacer />
              <EuiPopover
                id="trapFocus"
                ownFocus
                button={echoButton}
                isOpen={isEchoPopOverOpen}
                closePopover={() => {
                  setIsEchoPopOverOpen(false);
                  setEcho([0]);
                }}
              >
                <EuiComboBox
                  style={{ minWidth: 300 }}
                  placeholder="Choose echo content"
                  options={Array.from({ length: 256 }).map((_, index) => ({ label: index.toString() }))}
                  selectedOptions={echo.map(value => ({ label: value.toString() }))}
                  onChange={selectedOptions => setEcho(selectedOptions.map(({ label }) => parseInt(label)))}
                  isClearable={true}
                />

                <EuiSpacer />

                <EuiButton
                  isDisabled={echo.length === 0}
                  fill
                  onClick={() => axios.post('/api/echo', echo).then(({ data }) => console.log(`Echo result: ${data}`))}
                >
                  Send
                </EuiButton>
              </EuiPopover>
            </EuiPanel>
          </EuiPageContentBody>
        </EuiPageContent>
      </EuiPageBody>
    </EuiPage>
  );
};

ReactDOM.render(<IndexPage />, document.getElementById('root'));
