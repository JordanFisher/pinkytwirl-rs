import Foundation

class PinkyTwirl {
    private var engine: OpaquePointer?
    
    init(configPath: String) {
        engine = configPath.withCString { cString in
            pinky_twirl_engine_new(cString)
        }
    }
    
    deinit {
        if let engine = engine {
            pinky_twirl_engine_free(engine)
        }
    }
    
    func handleKeyEvent(keyCode: UInt16, down: Bool, shift: Bool, ctrl: Bool, option: Bool, meta: Bool, appName: String, windowName: String) -> [KeyEvent] {
        var length: UInt = 0
        let eventsPtr = appName.withCString { appNamePtr in
            windowName.withCString { windowNamePtr in
                pinky_twirl_engine_handle_key_event(
                    engine,
                    keyCode,
                    down,
                    shift,
                    ctrl,
                    option,
                    meta,
                    appNamePtr,
                    windowNamePtr,
                    &length
                )
            }
        }
        
        defer {
            pinky_twirl_free_key_events(eventsPtr, length)
        }
        
        guard let events = eventsPtr else {
            return []
        }
        
        return (0..<length).map { i in
            let event = events[Int(i)]
            return KeyEvent(
                key: String(cString: event.key),
                state: event.state,
                shift: event.shift,
                ctrl: event.ctrl,
                alt: event.alt,
                meta: event.meta
            )
        }
    }
}

struct KeyEvent {
    let key: String
    let state: Bool
    let shift: Bool
    let ctrl: Bool
    let alt: Bool
    let meta: Bool
}