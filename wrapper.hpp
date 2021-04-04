//
// Created by Alexander Czernay on 26.04.20.
//

// #include <IllustratorSDK.h>

#include "SPConfig.h"

// std library
//#include <stdio.h>
//#include <string.h>
//#include <vector>
//#include <sstream>
//#include <fstream>
//#include <algorithm>
//#include <math.h>

//#ifdef MAC_ENV
#include <CoreFoundation/CoreFoundation.h>
//#endif

//using namespace std;

// sweet pea headers
#include "SPTypes.h"
#include "SPBlocks.h"
#include "SPRuntme.h" // yes, the filename has no 'i'

// illustrator headers
#include "AITypes.h"
#include "AIArt.h"
#include "AIArtSet.h"
#include "AICSXSExtension.h"
#include "AIDictionary.h"
#include "AIDocument.h"
#include "AIHardSoft.h"
#include "AILayer.h"
#include "AILegacyTextConversion.h"
#include "AIMatchingArt.h"
#include "AIMdMemory.h"
#include "AIMenu.h"
#include "AIMenuGroups.h"
#include "AINotifier.h"
#include "AIPath.h"
#include "AIPathStyle.h"
#include "AIPlugin.h"
#include "AIPreference.h"
//#include "AITextFrame.h"
#include "AITimer.h"
#include "AITool.h"
#include "AIRuntime.h"
#include "AIUndo.h"
#include "AIUser.h"

#include "AIPanel.h"

// ATE text API
// #include "IText.h"

// SDK common headers
#include "Suites.hpp"
#include "Plugin.hpp"

// Extras
#include "AIFileFormat.h"
#include "AIControlBar.h"
#include "AIShapeConstruction.h"
#include "AIGeometry.h"
#include "AITransformArt.h"

// substituted classes
#include "subst.h"

// basic C wrapper classes
#include "src/wrapper.hpp"

// some stuff from Apple's foundation
//#include <Foundation/NSBundle.h>