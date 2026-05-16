import SceneKit

@_cdecl("scn_transaction_begin")
public func scn_transaction_begin() {
    SCNTransaction.begin()
}

@_cdecl("scn_transaction_commit")
public func scn_transaction_commit() {
    SCNTransaction.commit()
}

@_cdecl("scn_transaction_flush")
public func scn_transaction_flush() {
    SCNTransaction.flush()
}

@_cdecl("scn_transaction_get_animation_duration")
public func scn_transaction_get_animation_duration() -> Double {
    SCNTransaction.animationDuration
}

@_cdecl("scn_transaction_set_animation_duration")
public func scn_transaction_set_animation_duration(_ animationDuration: Double) {
    SCNTransaction.animationDuration = animationDuration
}

@_cdecl("scn_transaction_get_disable_actions")
public func scn_transaction_get_disable_actions() -> Bool {
    SCNTransaction.disableActions
}

@_cdecl("scn_transaction_set_disable_actions")
public func scn_transaction_set_disable_actions(_ disableActions: Bool) {
    SCNTransaction.disableActions = disableActions
}
