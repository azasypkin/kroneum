import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';
import {
  EuiButton,
  EuiFieldText,
  EuiFlexGroup,
  EuiFlexItem,
  EuiFormRow,
  EuiLoadingContent,
  EuiPage,
  EuiPageBody,
  EuiPageContent,
  EuiPageContentBody,
  EuiPageContentHeader,
  EuiPageContentHeaderSection,
  EuiPageHeader,
  EuiPageHeaderSection,
  EuiPanel,
  EuiPopover,
  EuiSpacer,
  EuiText,
  EuiTitle,
} from '@elastic/eui';
import axios from 'axios';
import { Note, Player } from './audio';

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

interface ADCStatus {
  isInProgress: boolean;
  response: number | null;
}
const DeviceADCSection = () => {
  const [adcStatus, setADCStatus] = useState<ADCStatus>({
    isInProgress: false,
    response: null,
  });

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

  return (
    <EuiFlexItem>
      <EuiPageContentHeader>
        <EuiPageContentHeaderSection>
          <EuiTitle>
            <h2>ADC</h2>
          </EuiTitle>
        </EuiPageContentHeaderSection>
      </EuiPageContentHeader>
      <EuiPageContentBody>
        <EuiPanel style={{ maxWidth: 300 }}>
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
      </EuiPageContentBody>
    </EuiFlexItem>
  );
};

interface EchoStatus {
  isInProgress: boolean;
  bytesString: string;
  isValid: boolean;
  response: number[] | null;
}
const DeviceDiagnosticsSection = () => {
  const [echoStatus, setEchoStatus] = useState<EchoStatus>({
    isInProgress: false,
    bytesString: '',
    isValid: false,
    response: null,
  });

  const showError = !echoStatus.isValid && echoStatus.bytesString.length > 0;

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
            <EuiButton
              onClick={async () => {
                const melody: Array<[Note, number]> = [
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

                Player.play(melody);

                await axios.post(
                  '/api/play',
                  melody.map(([note, duration]) => [note, duration * 400]),
                );
              }}
            >
              Play melody
            </EuiButton>
          </EuiFormRow>
          <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed">
            <EuiPopover
              id="trapFocus"
              ownFocus
              button={echoButton}
              isOpen={isEchoPopOverOpen}
              closePopover={() => {
                setIsEchoPopOverOpen(false);
                setEchoStatus({
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
                helpText={echoStatus.response ? `Response: [${echoStatus.response.join(', ')}]` : ''}
                isInvalid={showError}
                error={['Bytes should be a comma separated list of `u8` values.']}
              >
                <EuiFieldText
                  placeholder="Enter comma separated `u8` numbers..."
                  value={echoStatus.bytesString}
                  name="text"
                  isInvalid={showError}
                  onChange={ev =>
                    setEchoStatus({
                      ...echoStatus,
                      response: null,
                      isValid:
                        ev.target.value &&
                        ev.target.value.split(',').every(value => {
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
                      echoStatus.bytesString.split(',').map(value => parseInt(value.trim())),
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
                Send
              </EuiButton>
            </EuiPopover>
          </EuiFormRow>
        </EuiPanel>
      </EuiPageContentBody>
    </EuiFlexItem>
  );
};

interface RadioStatus {
  isInProgress: boolean;
  bytesString: string;
  isValid: boolean;
  response: number[] | null;
}
const DeviceRadioSection = () => {
  const [radioStatus, setRadioStatus] = useState<RadioStatus>({
    isInProgress: false,
    bytesString: '',
    isValid: false,
    response: null,
  });

  const showError = !radioStatus.isValid && radioStatus.bytesString.length > 0;

  const [isRadioPopOverOpen, setIsRadioPopOverOpen] = useState<boolean>(false);
  const radioButton = <EuiButton onClick={() => setIsRadioPopOverOpen(true)}>Send command</EuiButton>;

  return (
    <EuiFlexItem>
      <EuiPageContentHeader>
        <EuiPageContentHeaderSection>
          <EuiTitle>
            <h2>Radio</h2>
          </EuiTitle>
        </EuiPageContentHeaderSection>
      </EuiPageContentHeader>
      <EuiPageContentBody>
        <EuiPanel style={{ maxWidth: 300 }}>
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
                  onChange={ev =>
                    setRadioStatus({
                      ...radioStatus,
                      response: null,
                      isValid:
                        ev.target.value &&
                        ev.target.value.split(',').every(value => {
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
                      '/api/radio',
                      radioStatus.bytesString.split(',').map(value => parseInt(value.trim())),
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
            <DeviceADCSection />
            <DeviceRadioSection />
            <DeviceDiagnosticsSection />
          </EuiFlexGroup>
        </EuiPageContent>
      </EuiPageBody>
    </EuiPage>
  );
};

ReactDOM.render(<IndexPage />, document.getElementById('root'));
