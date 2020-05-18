import React, { useEffect, useState } from 'react';
import axios from 'axios';
import {
  EuiButton,
  EuiFieldNumber,
  EuiFlexItem,
  EuiFormRow,
  EuiLoadingContent,
  EuiPanel,
  EuiSpacer,
  EuiText,
  EuiSuperSelect,
} from '@elastic/eui';

export function FlashSection() {
  const [slots, setSlots] = useState<number[] | null>(null);
  useEffect(() => {
    axios.get('/api/flash').then(({ data }) => setSlots(data));
  }, []);

  const [flashContent, setFlashContent] = useState<{ slot: string; value: number }>({ slot: '0xaf', value: 0 });
  const [isWritingFlash, setIsWritingFlash] = useState<boolean>(false);

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
      <EuiSpacer />
      <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed" label="Write">
        <>
          <EuiFieldNumber
            placeholder="Enter u8 value to write to flash."
            value={flashContent.value}
            min={0}
            max={255}
            onChange={(ev) => {
              setFlashContent({ slot: flashContent.slot, value: parseInt(ev.target.value.trim()) || 0 });
            }}
            prepend={
              <EuiSuperSelect
                options={[
                  {
                    value: '0xaf',
                    inputDisplay: 'Config',
                    dropdownDisplay: 'Config',
                  },
                  {
                    value: '0x1f',
                    inputDisplay: 'Slot#1',
                    dropdownDisplay: 'Slot#1',
                  },
                  {
                    value: '0x2f',
                    inputDisplay: 'Slot#2',
                    dropdownDisplay: 'Slot#2',
                  },
                  {
                    value: '0x3f',
                    inputDisplay: 'Slot#3',
                    dropdownDisplay: 'Slot#3',
                  },
                  {
                    value: '0x4f',
                    inputDisplay: 'Slot#4',
                    dropdownDisplay: 'Slot#4',
                  },
                ]}
                valueOfSelected={flashContent.slot}
                onChange={(value) => {
                  setFlashContent({ slot: value, value: flashContent.value });
                }}
                hasDividers
              />
            }
            append={
              <EuiButton
                isDisabled={isWritingFlash}
                isLoading={isWritingFlash}
                fill
                onClick={() => {
                  setIsWritingFlash(true);
                  axios
                    .post('/api/flash/write', { slot: parseInt(flashContent.slot) || 0, value: flashContent.value })
                    .then(
                      () => setIsWritingFlash(false),
                      () => setIsWritingFlash(false),
                    );
                }}
              >
                Send
              </EuiButton>
            }
          />
        </>
      </EuiFormRow>
      <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed" label="Erase All">
        <EuiButton
          isDisabled={isWritingFlash}
          isLoading={isWritingFlash}
          fill
          onClick={() => {
            setIsWritingFlash(true);
            axios.post('/api/flash/erase').then(
              () => setIsWritingFlash(false),
              () => setIsWritingFlash(false),
            );
          }}
        >
          Erase
        </EuiButton>
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
