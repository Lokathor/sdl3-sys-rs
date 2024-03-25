#![allow(non_camel_case_types)]

use crate::stdinc::*;

#[repr(transparent)]
pub struct SDL_InitFlags(pub u32);

extern "C" {
  /// Initialize the SDL library.
  ///
  /// SDL_Init() simply forwards to calling SDL_InitSubSystem(). Therefore, the
  /// two may be used interchangeably. Though for readability of your code
  /// SDL_InitSubSystem() might be preferred.
  ///
  /// The file I/O (for example: SDL_IOFromFile) and threading
  /// (SDL_CreateThread) subsystems are initialized by default. Message boxes
  /// (SDL_ShowSimpleMessageBox) also attempt to work without initializing the
  /// video subsystem, in hopes of being useful in showing an error dialog when
  /// SDL_Init fails. You must specifically initialize other subsystems if you
  /// use them in your application.
  ///
  /// Logging (such as SDL_Log) works without initialization, too.
  ///
  /// `flags` may be any of the following OR'd together:
  ///
  /// - `SDL_INIT_TIMER`: timer subsystem
  /// - `SDL_INIT_AUDIO`: audio subsystem
  /// - `SDL_INIT_VIDEO`: video subsystem; automatically initializes the events
  ///   subsystem
  /// - `SDL_INIT_JOYSTICK`: joystick subsystem; automatically initializes the
  ///   events subsystem
  /// - `SDL_INIT_HAPTIC`: haptic (force feedback) subsystem
  /// - `SDL_INIT_GAMEPAD`: gamepad subsystem; automatically initializes the
  ///   joystick subsystem
  /// - `SDL_INIT_EVENTS`: events subsystem
  /// - `SDL_INIT_SENSOR`: sensor subsystem
  ///
  /// Subsystem initialization is ref-counted, you must call SDL_QuitSubSystem()
  /// for each SDL_InitSubSystem() to correctly shutdown a subsystem manually
  /// (or call SDL_Quit() to force shutdown). If a subsystem is already loaded
  /// then this call will increase the ref-count and return.
  ///
  /// * `flags`: subsystem initialization flags
  ///
  /// **Returns:** 0 on success or a negative error code on failure; call
  /// SDL_GetError() for more information.
  ///
  /// **Since:** This function is available since SDL 3.0.0.
  ///
  /// * **See Also:** [SDL_InitSubSystem]
  /// * **See Also:** [SDL_Quit]
  /// * **See Also:** [SDL_SetMainReady]
  /// * **See Also:** [SDL_WasInit]
  pub fn SDL_Init(flags: SDL_InitFlags) -> c_int;

  /// Compatibility function to initialize the SDL library.
  ///
  /// This function and SDL_Init() are interchangeable.
  ///
  /// * `flags`: any of the flags used by SDL_Init(); see SDL_Init for details.
  ///
  /// **Returns:** 0 on success or a negative error code on failure; call
  /// SDL_GetError() for more information.
  ///
  /// **Since:** This function is available since SDL 3.0.0.
  ///
  /// * **See Also:** [SDL_Init]
  /// * **See Also:** [SDL_Quit]
  /// * **See Also:** [SDL_QuitSubSystem]
  pub fn SDL_InitSubSystem(flags: SDL_InitFlags) -> c_int;

  /// Shut down specific SDL subsystems.
  ///
  /// You still need to call SDL_Quit() even if you close all open subsystems
  /// with SDL_QuitSubSystem().
  ///
  /// * `flags`: any of the flags used by SDL_Init(); see SDL_Init for details.
  ///
  /// **Since:** This function is available since SDL 3.0.0.
  ///
  /// * **See Also:** [SDL_InitSubSystem]
  /// * **See Also:** [SDL_Quit]
  pub fn SDL_QuitSubSystem(flags: SDL_InitFlags) -> c_int;

  /// Get a mask of the specified subsystems which are currently initialized.
  ///
  /// * `flags`: any of the flags used by SDL_Init(); see SDL_Init for details.
  ///
  /// **Returns:** a mask of all initialized subsystems if `flags` is 0,
  /// otherwise it returns the initialization status of the specified
  /// subsystems.
  ///
  /// **Since:** This function is available since SDL 3.0.0.
  ///
  /// * **See Also:** [SDL_Init]
  /// * **See Also:** [SDL_InitSubSystem]
  pub fn SDL_WasInit(flags: SDL_InitFlags) -> SDL_InitFlags;

  /// Clean up all initialized subsystems.
  ///
  /// You should call this function even if you have already shutdown each
  /// initialized subsystem with SDL_QuitSubSystem(). It is safe to call this
  /// function even in the case of errors in initialization.
  ///
  /// You can use this function with atexit() to ensure that it is run when your
  /// application is shutdown, but it is not wise to do this from a library or
  /// other dynamically loaded code.
  ///
  /// **Since:** This function is available since SDL 3.0.0.
  ///
  /// * **See Also:** [SDL_Init]
  /// * **See Also:** [SDL_QuitSubSystem]
  pub fn SDL_Quit();
}
