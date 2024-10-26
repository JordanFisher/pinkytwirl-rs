import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem?
    private var eventTap: CFMachPort?
    private let sourceInput = CGEventSource(stateID: .hidSystemState)
    private var accessibilityTimer: Timer?
    private var engine: PinkyTwirlEngine?

    func applicationDidFinishLaunching(_ notification: Notification) {
        // setupStatusBarItem()
        checkAndRequestAccessibilityPermissions()
        engine = PinkyTwirlEngine.new("testpath")
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

        // setupEventTap()
    }
}