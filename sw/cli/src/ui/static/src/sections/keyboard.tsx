import React, { useCallback, useState } from 'react';
import axios from 'axios';
import {
  EuiButton,
  EuiFlexItem,
  EuiFormRow,
  EuiText,
  EuiLink,
  EuiPanel,
  EuiSpacer,
  EuiFieldNumber,
  EuiPopover,
  EuiButtonEmpty,
  EuiFlexGroup,
  EuiSwitch,
} from '@elastic/eui';

interface Modifiers {
  leftCtrl: boolean;
  leftShift: boolean;
  leftAlt: boolean;
  leftGUI: boolean;
  rightCtrl: boolean;
  rightShift: boolean;
  rightAlt: boolean;
  rightGUI: boolean;
}

export function KeyboardSection() {
  const [loadingState, setLoadingState] = useState<{ keyCode?: boolean; mediaKey?: boolean }>({});
  const [keyCode, setKeyCode] = useState<number>(57);
  const [delay, setDelay] = useState<number>(0);
  const [isModifiersPopoverOpen, setIsModifiersPopoverOpen] = useState<boolean>(false);
  const [modifiers, setModifiers] = useState<Modifiers>({
    leftCtrl: false,
    leftShift: false,
    leftAlt: false,
    leftGUI: false,
    rightCtrl: false,
    rightShift: false,
    rightAlt: false,
    rightGUI: false,
  });

  const onMediaKey = useCallback((mediaKey: number) => {
    setLoadingState({ mediaKey: true });
    axios
      .post('/api/media_key', { keyCode: mediaKey, delay })
      .then(() => setLoadingState({ mediaKey: false }))
      .catch(() => setLoadingState({ mediaKey: false }));
  }, []);

  const isDelayValid = delay >= 0 && delay <= 2;
  const isKeyCodeValid = keyCode >= 0 && keyCode <= 255;
  const mediaKeyDisabled = loadingState.keyCode || loadingState.mediaKey || !isDelayValid;

  return (
    <EuiFlexItem>
      <EuiSpacer />
      <EuiPanel>
        <EuiFormRow style={{ alignItems: 'center' }} display="columnCompressed" label="Delay">
          <EuiFieldNumber
            placeholder="Enter delay in seconds."
            value={delay}
            min={0}
            max={2}
            onChange={(ev) => setDelay(parseInt(ev.target.value.trim()) || 0)}
          />
        </EuiFormRow>
        <EuiSpacer />
        <EuiFormRow
          style={{ alignItems: 'center' }}
          display="columnCompressed"
          label="Keyboard key"
          helpText={
            <EuiText size="xs">
              See the list of key codes{' '}
              <EuiLink href="https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf" target="_blank">
                here
              </EuiLink>
              .
            </EuiText>
          }
        >
          <>
            <EuiFieldNumber
              placeholder="Enter u8 key code."
              value={keyCode}
              min={0}
              max={255}
              onChange={(ev) => {
                setKeyCode(parseInt(ev.target.value.trim()) || 0);
              }}
              prepend={
                <EuiPopover
                  panelPaddingSize="s"
                  isOpen={isModifiersPopoverOpen}
                  closePopover={() => setIsModifiersPopoverOpen(false)}
                  button={
                    <EuiButtonEmpty size="xs" onClick={() => setIsModifiersPopoverOpen(!isModifiersPopoverOpen)}>
                      Modifiers
                    </EuiButtonEmpty>
                  }
                >
                  <div>
                    <EuiFlexGroup wrap={true} direction="column" gutterSize="s" responsive={false}>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Left Ctrl'}
                          checked={modifiers.leftCtrl}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              leftCtrl: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Left Shift'}
                          checked={modifiers.leftShift}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              leftShift: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Left Alt'}
                          checked={modifiers.leftAlt}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              leftAlt: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Left GUI'}
                          checked={modifiers.leftGUI}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              leftGUI: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Right Ctrl'}
                          checked={modifiers.rightCtrl}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              rightCtrl: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Right Shift'}
                          checked={modifiers.rightShift}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              rightShift: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Right Alt'}
                          checked={modifiers.rightAlt}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              rightAlt: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                      <EuiFlexItem grow={false}>
                        <EuiSwitch
                          compressed
                          label={'Right GUI'}
                          checked={modifiers.rightGUI}
                          onChange={(e) =>
                            setModifiers({
                              ...modifiers,
                              rightGUI: e.target.checked,
                            })
                          }
                        />
                      </EuiFlexItem>
                    </EuiFlexGroup>
                  </div>
                </EuiPopover>
              }
              append={
                <EuiButton
                  isDisabled={loadingState.mediaKey || loadingState.keyCode || !isKeyCodeValid || !isDelayValid}
                  isLoading={loadingState.keyCode}
                  fill
                  onClick={() => {
                    setLoadingState({ keyCode: true });
                    axios
                      .post('/api/key', { keyCode, delay, modifiers })
                      .then(() => setLoadingState({ keyCode: false }));
                  }}
                >
                  Send
                </EuiButton>
              }
            />
          </>
        </EuiFormRow>
        <EuiSpacer />
        <EuiFormRow display="columnCompressed" style={{ alignItems: 'center' }} label="Media controls">
          <EuiFlexGroup direction="row" justifyContent="spaceBetween" wrap={true}>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x01)}>
                Volume Up
              </EuiButton>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x02)}>
                Volume Down
              </EuiButton>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x04)}>
                Mute
              </EuiButton>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x08)}>
                Next Track
              </EuiButton>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x10)}>
                Previous Track
              </EuiButton>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x20)}>
                Play/Pause
              </EuiButton>
            </EuiFlexItem>
            <EuiFlexItem>
              <EuiButton isDisabled={mediaKeyDisabled} onClick={() => onMediaKey(0x40)}>
                Stop
              </EuiButton>
            </EuiFlexItem>
          </EuiFlexGroup>
        </EuiFormRow>
      </EuiPanel>
    </EuiFlexItem>
  );
}
