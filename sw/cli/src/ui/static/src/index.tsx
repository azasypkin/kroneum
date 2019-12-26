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
  EuiLoadingContent,
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

const DeviceIdentifierSection = ({ identifier }: { identifier?: DeviceInfo['identifier'] }) => {
  const content = identifier ? (
    <EuiPanel style={{ maxWidth: 300 }}>
      <EuiFormRow label="Bus" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{identifier?.bus ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Address" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{identifier?.address ?? 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Vendor ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{identifier ? `0x${identifier.vendorID.toString(16).toUpperCase()}` : 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Product ID" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{identifier ? `0x${identifier.productID.toString(16).toUpperCase()}` : 'Unknown'}</EuiText>
      </EuiFormRow>
      <EuiFormRow label="Manufacturer" display="columnCompressed" style={{ alignItems: 'center' }}>
        <EuiText size="s">{identifier?.manufacturer ?? 'Unknown'}</EuiText>
      </EuiFormRow>
    </EuiPanel>
  ) : (
    <EuiPanel style={{ maxWidth: 300 }}>
      <EuiLoadingContent lines={5} />
    </EuiPanel>
  );

  return (
    <EuiFlexItem>
      <EuiPageContentHeader>
        <EuiPageContentHeaderSection>
          <EuiTitle>
            <h2>Device information</h2>
          </EuiTitle>
        </EuiPageContentHeaderSection>
      </EuiPageContentHeader>
      <EuiPageContentBody>{content}</EuiPageContentBody>
    </EuiFlexItem>
  );
};

const DeviceFlashContentSection = ({ slots }: { slots?: number[] }) => {
  const content = slots ? (
    <EuiPanel style={{ maxWidth: 300 }}>
      {slots.map((slotContent, slotIndex) => {
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
  ) : (
    <EuiPanel style={{ maxWidth: 300 }}>
      <EuiLoadingContent lines={5} />
    </EuiPanel>
  );
  return (
    <EuiFlexItem>
      <EuiPageContentHeader>
        <EuiPageContentHeaderSection>
          <EuiTitle>
            <h2>Flash slots</h2>
          </EuiTitle>
        </EuiPageContentHeaderSection>
      </EuiPageContentHeader>
      <EuiPageContentBody>{content}</EuiPageContentBody>
    </EuiFlexItem>
  );
};

const DeviceAlarmSection = () => {
  return (
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
      </EuiPageContentBody>
    </EuiFlexItem>
  );
};

const DeviceDiagnosticsSection = () => {
  const [echoByteSequence, setEchoByteSequence] = useState<number[]>([0]);
  const [isEchoPopOverOpen, setIsEchoPopOverOpen] = useState<boolean>(false);

  const echoButton = <EuiButton onClick={() => setIsEchoPopOverOpen(true)}>Send echo</EuiButton>;

  return (
    <EuiFlexItem>
      <EuiPageContentHeader>
        <EuiPageContentHeaderSection>
          <EuiTitle>
            <h2>Diagnostics</h2>
          </EuiTitle>
        </EuiPageContentHeaderSection>
      </EuiPageContentHeader>
      <EuiPageContentBody>
        <EuiPanel style={{ maxWidth: 300 }}>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiButton onClick={() => axios.get('/api/beep')}>Send beep</EuiButton>
          </EuiFormRow>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiButton onClick={() => axios.get('/api/melody')}>Play melody</EuiButton>
          </EuiFormRow>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiPopover
              id="trapFocus"
              ownFocus
              button={echoButton}
              isOpen={isEchoPopOverOpen}
              closePopover={() => {
                setIsEchoPopOverOpen(false);
                setEchoByteSequence([0]);
              }}
            >
              <EuiComboBox
                style={{ minWidth: 300 }}
                placeholder="Choose echo content"
                options={Array.from({ length: 256 }).map((_, index) => ({ label: index.toString() }))}
                selectedOptions={echoByteSequence.map(value => ({ label: value.toString() }))}
                onChange={selectedOptions => setEchoByteSequence(selectedOptions.map(({ label }) => parseInt(label)))}
                isClearable={true}
              />

              <EuiSpacer />

              <EuiButton
                isDisabled={echoByteSequence.length === 0}
                fill
                onClick={() =>
                  axios.post('/api/echo', echoByteSequence).then(({ data }) => console.log(`Echo result: ${data}`))
                }
              >
                Send
              </EuiButton>
            </EuiPopover>
          </EuiFormRow>
        </EuiPanel>
      </EuiPageContentBody>
    </EuiFlexItem>
  );
};

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
            <DeviceIdentifierSection identifier={info?.identifier} />
            <DeviceFlashContentSection slots={info?.flash} />
            <DeviceAlarmSection />
            <DeviceDiagnosticsSection />
          </EuiFlexGroup>
        </EuiPageContent>
      </EuiPageBody>
    </EuiPage>
  );
};

ReactDOM.render(<IndexPage />, document.getElementById('root'));
