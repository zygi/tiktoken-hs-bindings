import Distribution.Simple
import Distribution.Simple.Setup
import Distribution.Types.HookedBuildInfo
import Distribution.Types.PackageDescription
import Distribution.Types.LocalBuildInfo

import System.Directory
import System.FilePath
import System.Process
import System.Exit

main = defaultMainWithHooks simpleUserHooks { postConf = postConfImpl }

postConfImpl :: Args -> ConfigFlags -> PackageDescription -> LocalBuildInfo -> IO ()
postConfImpl args flags pkg_descr localBuildInfo = do
  -- First, build the library with cargo
  _ <- readProcess "bash" ["-c", "pwd && cd rust-src && cargo build --release"] ""

  let libPath = dataDir pkg_descr </> "rust-src/target/release"

  -- Now, let's copy the library to the build directory
  copyFile (libPath </> "libtiktoken_hs_bindings_native.a") ((buildDir localBuildInfo) </> "libCtiktoken_hs_bindings_native.a")
  copyFile (libPath </> "libtiktoken_hs_bindings_native.so") ((buildDir localBuildInfo) </> ("libtiktoken_hs_bindings_native.so"))
  

