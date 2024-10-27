import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem?
    private var eventTap: CFMachPort?
    private let sourceInput = CGEventSource(stateID: .hidSystemState)
    private var accessibilityTimer: Timer?
    private var engine: PinkyTwirlEngine?

    func applicationDidFinishLaunching(_ notification: Notification) {
        setupStatusBarItem()
        checkAndRequestAccessibilityPermissions()
        engine = PinkyTwirlEngine.new("../../../pinkytwirl-rs/src/user_config")
    }

    private func setupStatusBarItem() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        statusItem?.button?.title = "⌨️"
        let menu = NSMenu()
        menu.addItem(NSMenuItem(title: "Enable", action: #selector(toggleEnabled), keyEquivalent: "e"))
        menu.addItem(NSMenuItem.separator())
        menu.addItem(NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q"))
    }

    @objc private func toggleEnabled() {
        if let eventTap = eventTap {
            let isEnabled = CGEvent.tapIsEnabled(tap: eventTap)
            CGEvent.tapEnable(tap: eventTap, enable: !isEnabled)
            print("PinkyTwirl is \(isEnabled ? "disabled" : "enabled")")
            statusItem?.button?.title = isEnabled ? "⌨️" : "🚫"
            statusItem?.menu?.items.first?.title = isEnabled ? "Disable" : "Enable"
        }
    }

    @objc func checkAndRequestAccessibilityPermissions() {
        if AXIsProcessTrusted() {
            print("Accessibility permissions already granted")
        } else {
            print("Accessibility permissions not yet granted, please grant them.")

            // We don't yet have permissions, so let's ask for them.
            NSWorkspace.shared.open(URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")!)
            // Poll until we have permissions.
            accessibilityTimer?.invalidate()
            accessibilityTimer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { [weak self] _ in
                if AXIsProcessTrusted() {
                    print("Accessibility permissions granted")
                    self?.accessibilityTimer?.invalidate()
                } else {
                    print("Accessibility permissions not yet granted. Waiting...")
                }
            }
        }

        setupEventTap()
    }

    private func setupEventTap() {
        let eventMask = (
            (1 << CGEventType.keyDown.rawValue) |
            (1 << CGEventType.keyUp.rawValue) |
            (1 << CGEventType.flagsChanged.rawValue))
        
        guard let eventTap = CGEvent.tapCreate(
            tap: .cgSessionEventTap,
            place: .headInsertEventTap,
            options: .defaultTap,
            eventsOfInterest: CGEventMask(eventMask),
            callback: { (proxy, type, event, refcon) in
                let unmanagedSelf = Unmanaged<AppDelegate>.fromOpaque(refcon!)
                let appDelegate = unmanagedSelf.takeUnretainedValue()
                return appDelegate.handleEvent(proxy: proxy, type: type, event: event)
            },
            userInfo: UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque())
        ) else {
            print("Failed to create event tap")
            return
        }

        self.eventTap = eventTap

        let runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0)
        CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
        CGEvent.tapEnable(tap: eventTap, enable: true)
        print("Event tap enabled")
    }

    private func getActiveWindowInfo() -> (appName: String, windowTitle: String, bundleId: String) {
        guard let app = NSWorkspace.shared.frontmostApplication else {
            return ("unknown", "unknown", "unknown")
        }

        let appName = app.localizedName ?? "unknown"
        let bundleId = app.bundleIdentifier ?? "unknown"

        // Get the title of the focused window.
        var windowTitle = "unknown"
        let appRef = AXUIElementCreateApplication(app.processIdentifier)
        var windowRef: CFTypeRef?
        let focusedWindow = AXUIElementCopyAttributeValue(appRef, kAXFocusedWindowAttribute as CFString, &windowRef)
        if focusedWindow == .success {
            var titleRef: CFTypeRef?
            let title = AXUIElementCopyAttributeValue(windowRef as! AXUIElement, kAXTitleAttribute as CFString, &titleRef)
            if title == .success {
                windowTitle = titleRef as! String
            }
        }

        return (appName, windowTitle, bundleId)
    }

    private func handleEvent(proxy: CGEventTapProxy, type: CGEventType, event: CGEvent) -> Unmanaged<CGEvent>? {
        // Ignore our own synthetic events.
        if event.getIntegerValueField(.eventSourceUserData) == 0x1234 {
            return Unmanaged.passRetained(event)
        }

        let keyCode = event.getIntegerValueField(.keyboardEventKeycode)
        let flags = event.flags

        // Get active window info.
        let (appName, windowTitle, bundleId) = getActiveWindowInfo()

        // Debug print the event.
        print("Event: \(type) \(keyCode) \(flags) \(appName) \(windowTitle) \(bundleId)")

        var isKeyDown = false
        if type == .flagsChanged {
            // This is when a modifier key is pressed or released.
            // We want to figure out if this is a key down or key up event.
            // We can do this by checking the current state of the key.
            isKeyDown = (
                // Left side modifier keys.
                flags.contains(.maskCommand) && keyCode == 55 ||
                flags.contains(.maskControl) && keyCode == 59 ||
                flags.contains(.maskShift) && keyCode == 56 ||
                flags.contains(.maskAlternate) && keyCode == 58 ||
                // Right side modifier keys.
                flags.contains(.maskCommand) && keyCode == 54 ||
                flags.contains(.maskControl) && keyCode == 62 ||
                flags.contains(.maskShift) && keyCode == 60 ||
                flags.contains(.maskAlternate) && keyCode == 61)
        } else {
            isKeyDown = type == .keyDown
        }

        // Handle the event.
        let shouldSuppress = engine?.macos_handle_key_event(
            keyCode,
            isKeyDown,
            flags.contains(.maskShift),
            flags.contains(.maskControl),
            flags.contains(.maskAlternate),
            flags.contains(.maskCommand),
            appName,
            windowTitle)
        
        print("shouldSuppress: \(shouldSuppress!)")        
        let synthetic_keys = engine?.get_synthetic_keys()
        for i in 0..<result!.len() {
            print("Synthetic key to generate \(i): \(result![i])")
        }

        if shouldSuppress! {
            return nil
        } else {
            return Unmanaged.passRetained(event)
        }

            // case .keyDown:
            //     if keyCode == 38 {
            //         if let synth = CGEvent(keyboardEventSource: nil, virtualKey: 123, keyDown: true) {
            //             synth.flags.insert([.maskShift, .maskAlternate])
            //             synth.setIntegerValueField(.eventSourceUserData, value: 0x1234)
            //             synth.post(tap: .cgSessionEventTap)
            //         }
            //         if let synth = CGEvent(keyboardEventSource: nil, virtualKey: 123, keyDown: false) {
            //             synth.flags.insert([.maskShift, .maskAlternate])
            //             synth.setIntegerValueField(.eventSourceUserData, value: 0x1234)
            //             synth.post(tap: .cgSessionEventTap)
            //         }
            //         return nil
            //     }
    }
}