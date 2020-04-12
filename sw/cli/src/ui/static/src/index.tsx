import React from 'react';
import ReactDOM from 'react-dom';

import '@elastic/eui/dist/eui_theme_dark.css';
import {
  EuiPage,
  EuiPageBody,
  EuiPageContent,
  EuiPageHeader,
  EuiPageHeaderSection,
  EuiTabbedContent,
  EuiTitle,
} from '@elastic/eui';

import {
  ADCSection,
  AlarmsSection,
  DiagnosticsSection,
  InfoSection,
  FlashSection,
  KeyboardSection,
  RadioSection,
} from './sections';

const IndexPage = () => {
  const tabs = [
    {
      id: 'id',
      name: 'Device',
      content: <InfoSection />,
    },
    {
      id: 'flash',
      name: 'Flash',
      content: <FlashSection />,
    },
    {
      id: 'alarms',
      name: 'Alarms',
      content: <AlarmsSection />,
    },
    {
      id: 'adc',
      name: 'ADC',
      content: <ADCSection />,
    },
    {
      id: 'radio',
      name: 'Radio',
      content: <RadioSection />,
    },
    {
      id: 'keyboard',
      name: 'Keyboard',
      content: <KeyboardSection />,
    },
    {
      id: 'diagnostics',
      name: 'Diagnostics',
      content: <DiagnosticsSection />,
    },
  ];

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
          <EuiTabbedContent tabs={tabs} initialSelectedTab={tabs[0]} autoFocus="selected" />
        </EuiPageContent>
      </EuiPageBody>
    </EuiPage>
  );
};

ReactDOM.render(<IndexPage />, document.getElementById('root'));
