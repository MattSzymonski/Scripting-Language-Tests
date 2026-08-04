# DLL Analysis: UnrealEditor-RedRoads.dll

## File

- **Path**: `Z:\Perforce\mateusz.szymonski_WX.Dev\Engine\Plugins\RED\RedRoads\Binaries\Win64\UnrealEditor-RedRoads.dll`
- **Size**: 479,744 bytes
- **Entropy**: 6.372 (whole file, 0-8 scale)
- **MD5**: `6a7b3e589d3b678d60875e9cac7550b9`
- **SHA1**: `c6cba0454906c315ba3ab6845d29d7deeb524204`
- **SHA256**: `17c62385c30b6f5656b88abff21943396da8b9a5c266c3076bf12aa3e6dca8c8`

## Headers

- **Machine**: x64 (AMD64)
- **Is DLL**: True
- **Timestamp (UTC)**: 2026-07-10T14:09:01+00:00
- **File characteristics**: EXECUTABLE_IMAGE, LARGE_ADDRESS_AWARE, DLL
- **Subsystem**: WINDOWS_GUI (v6.0)
- **Linker version**: 14.44
- **Image base**: 0x180000000   **Size of image**: 503,808
- **Entry point RVA**: 0x0002BE20
- **Checksum declared**: 0x00000000  **valid**: False
- **DLL characteristics**: HIGH_ENTROPY_VA, DYNAMIC_BASE, NX_COMPAT

## Sections (7)

| Name | Perms | Raw Size | Virtual Size | Entropy |
|---|---|---|---|---|
| .text | R-X | 186,880 | 186,812 | 6.029 |
| .uedbg | R-X | 4,608 | 4,268 | 5.673 |
| .rdata | R-- | 235,520 | 235,514 | 5.609 |
| .data | RW- | 3,584 | 7,120 | 1.726 |
| .pdata | R-- | 10,240 | 10,056 | 5.411 |
| .rsrc | RW- | 25,088 | 25,080 | 7.331 |
| .reloc | R-- | 12,800 | 12,632 | 5.433 |

## Imports (13 DLLs, 1180 functions)

| DLL | Kind | Function Count |
|---|---|---|
| UnrealEditor-CoreUObject.dll | module | 118 |
| UnrealEditor-Engine.dll | module | 712 |
| UnrealEditor-UnrealEd.dll | module | 2 |
| UnrealEditor-DeveloperSettings.dll | module | 11 |
| UnrealEditor-Core.dll | module | 90 |
| UnrealEditor-RedUniversalSpline.dll | module | 64 |
| UnrealEditor-RedSplineTool.dll | module | 139 |
| KERNEL32.dll | system | 18 |
| VCRUNTIME140.dll | system | 9 |
| VCRUNTIME140_1.dll | system | 1 |
| api-ms-win-crt-string-l1-1-0.dll | system | 2 |
| api-ms-win-crt-math-l1-1-0.dll | system | 2 |
| api-ms-win-crt-runtime-l1-1-0.dll | system | 12 |

<details><summary>UnrealEditor-CoreUObject.dll — 118 functions</summary>

```
?GetObjectsOfClass@@YAXPEBVUClass@@AEAV?$TArray@PEAVUObject@@V?$TSizedDefaultAllocator@$0CA@@@@@_NW4EObjectFlags@@W4EInternalObjectFlags@@@Z
?GetAsyncLoadingInternalFlagsExclusion@UE@@YA?AW4EInternalObjectFlags@@XZ
?GetTransientPackage@@YAPEAVUPackage@@XZ
?IsA@FObjectPtr@@QEBA_NPEBVUClass@@@Z
?SetRootFlags@FUObjectItem@@AEAA_NW4EInternalObjectFlags@@@Z
?ClearRootFlags@FUObjectItem@@AEAA_NW4EInternalObjectFlags@@@Z
?GetPathName@UObjectBaseUtility@@QEBA?AVFString@@PEBVUObject@@@Z
?MarkPackageDirty@UObjectBaseUtility@@QEBA_NXZ
?GetTypedOuter@UObjectBaseUtility@@QEBAPEAVUObject@@PEAVUClass@@@Z
??0FStaticConstructObjectParameters@@QEAA@PEBVUClass@@@Z
?AssertIfInConstructor@FObjectInitializer@@SAXPEAVUObject@@PEB_W@Z
?StaticConstructObject_Internal@@YAPEAVUObject@@AEBUFStaticConstructObjectParameters@@@Z
?GUObjectArray@@3VFUObjectArray@@A
?HandleReadCallbackQuantity@Private@CoreUObject@UE@@3U?$atomic@H@std@@A
?GIsIncrementalReachabilityPending@GC@UE@@3_NA
?CheckDefaultSubobjectsInternal@UObject@@MEBA_NXZ
?ProcessEvent@UObject@@UEAAXPEAVUFunction@@PEAX@Z
?BuildSubobjectMapping@UObject@@UEBAXPEAV1@AEAV?$TMap@PEAVUObject@@PEAV1@VFDefaultSetAllocator@@U?$TDefaultMapHashableKeyFuncs@PEAVUObject@@PEAV1@$0A@@@@@@Z
?IsDataValid@UObject@@UEBA?AW4EDataValidationResult@@AEAVFDataValidationContext@@@Z
?PreDestroyFromReplication@UObject@@UEAAXXZ
?PostNetReceive@UObject@@UEAAXXZ
?PreNetReceive@UObject@@UEAAXXZ
?IsSupportedForNetworking@UObject@@UEBA_NXZ
?IsFullNameStableForNetworking@UObject@@UEBA_NXZ
?IsNameStableForNetworking@UObject@@UEBA_NXZ
?RegisterReplicationFragments@UObject@@UEAAXAEAVFFragmentRegistrationContext@Net@UE@@W4EFragmentRegistrationFlags@34@@Z
?IsSelected@UObject@@QEBA_NXZ
?GetReplicatedCustomConditionState@UObject@@UEBAXAEAVFCustomPropertyConditionState@@@Z
?GetLifetimeReplicatedProps@UObject@@UEBAXAEAV?$TArray@VFLifetimeProperty@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?TagSubobjects@UObject@@UEAAXW4EObjectFlags@@@Z
?IsSafeForRootSet@UObject@@UEBA_NXZ
?IsLocalizedResource@UObject@@UEBA_NXZ
?GetPrimaryAssetId@UObject@@UEBA?AUFPrimaryAssetId@@XZ
?IsAsset@UObject@@UEBA_NXZ
?GetAssetRegistryTagMetadata@UObject@@UEBAXAEAV?$TMap@VFName@@UFAssetRegistryTagMetadata@UObject@@VFDefaultSetAllocator@@U?$TDefaultMapHashableKeyFuncs@VFName@@UFAssetRegistryTagMetadata@UObject@@$0A@@@@@@Z
?GetExtendedAssetRegistryTagsForSave@UObject@@UEBAXPEBVITargetPlatform@@AEAV?$TArray@UFAssetRegistryTag@UObject@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetAdditionalAssetDataObjectsForCook@UObject@@UEBAXAEAUFArchiveCookContext@@AEAV?$TArray@PEAVUObject@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetAssetRegistryTags@UObject@@UEBAXVFAssetRegistryTagsContext@@@Z
?GetAssetRegistryTags@UObject@@UEBAXAEAV?$TArray@UFAssetRegistryTag@UObject@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetResourceSizeEx@UObject@@UEAAXAEAUFResourceSizeEx@@@Z
?ImplementsGetWorld@UObject@@UEBA_NXZ
?GetWorld@UObject@@UEBAPEAVUWorld@@XZ
?Rename@UObject@@UEAA_NPEB_WPEAV1@I@Z
?GetPreloadDependencies@UObject@@UEAAXAEAV?$TArray@PEAVUObject@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?NeedsLoadForTargetPlatform@UObject@@UEBA_NPEBVITargetPlatform@@@Z
?NeedsLoadForServer@UObject@@UEBA_NXZ
?NeedsLoadForClient@UObject@@UEBA_NXZ
?PostRename@UObject@@UEAAXPEAV1@VFName@@@Z
?IsSelectedInEditor@UObject@@MEBA_NXZ
?PostTransacted@UObject@@UEAAXAEBVFTransactionObjectEvent@@@Z
?PostEditUndo@UObject@@UEAAXXZ
?PostEditUndo@UObject@@UEAAXV?$TSharedPtr@VITransactionObjectAnnotation@@$00@@@Z
?PreEditUndo@UObject@@UEAAXXZ
?PostEditChangeChainProperty@UObject@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?PostEditChangeProperty@UObject@@UEAAXAEAUFPropertyChangedEvent@@@Z
?CanEditChange@UObject@@UEBA_NPEBVFProperty@@@Z
?CanEditChange@UObject@@UEBA_NAEBVFEditPropertyChain@@@Z
?PreEditChange@UObject@@UEAAXPEAVFProperty@@@Z
?PreEditChange@UObject@@UEAAXAEAVFEditPropertyChain@@@Z
?Serialize@UObject@@UEAAXAEAVFArchive@@@Z
?Serialize@UObject@@UEAAXVFStructuredArchiveRecord@@@Z
?FinishDestroy@UObject@@UEAAXXZ
?BeginDestroy@UObject@@UEAAXXZ
?PostLoadSubobjects@UObject@@UEAAXPEAUFObjectInstancingGraph@@@Z
?PostLoad@UObject@@UEAAXXZ
?IsCapturingAsRootObjectForTransaction@UObject@@UEBA_NXZ
?Modify@UObject@@UEAA_N_N@Z
?CollectSaveOverrides@UObject@@UEAAXVFObjectCollectSaveOverridesContext@@@Z
?PreSave@UObject@@UEAAXVFObjectPreSaveContext@@@Z
?PostSaveRoot@UObject@@UEAAXVFObjectPostSaveRootContext@@@Z
?PreSaveRoot@UObject@@UEAAXVFObjectPreSaveRootContext@@@Z
?PostReinitProperties@UObject@@UEAAXXZ
?PostInitProperties@UObject@@UEAAXXZ
?GetVersePath@UObjectBaseUtility@@UEBA?AVFVersePath@Core@UE@@XZ
?CreateCluster@UObjectBaseUtility@@UEAAXXZ
?CanBeInCluster@UObjectBaseUtility@@UEBA_NXZ
?GetFNameForStatID@UObjectBase@@UEBA?AVFName@@XZ
?DeferredRegister@UObjectBase@@MEAAXPEAVUClass@@PEB_W1@Z
?Z_Construct_UScriptStruct_FGuid@@YAPEAVUScriptStruct@@W4ETypeConstructPhase@@@Z
?Z_Construct_UScriptStruct_FColor@@YAPEAVUScriptStruct@@W4ETypeConstructPhase@@@Z
?Z_Construct_UScriptStruct_FVector@@YAPEAVUScriptStruct@@W4ETypeConstructPhase@@@Z
?Z_Construct_UScriptStruct_FLinearColor@@YAPEAVUScriptStruct@@W4ETypeConstructPhase@@@Z
?CastLogError@@YAXPEB_W0@Z
?GetOrCreateIDForObject@FSoftObjectPath@@SA?AU1@UFObjectPtr@@@Z
?Z_Construct_UScriptStruct_FDirectoryPath@@YAPEAVUScriptStruct@@W4ETypeConstructPhase@@@Z
?GetPrivateStaticClassBody@@YAXPEB_W0AEAPEAVUClass@@P6AXXZIIW4EClassFlags@@W4EClassCastFlags@@0P6AXAEBVFObjectInitializer@@@ZP6APEAVUObject@@AEAVFVTableHelper@@@Z$$QEAUFUObjectCppClassStaticFunctions@@P6APEAV1@XZP6APEAV1@XZ@Z
?InternalCreateDefaultObjectWrapper@UClass@@AEBAXXZ
?IsChildOf@UStruct@@QEBA_NPEBV1@@Z
?Z_Construct_UClass_UScriptStruct@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
??4FWeakObjectPtr@@QEAAXUFObjectPtr@@@Z
?CallFunctionByNameWithArguments@UObject@@QEAA_NPEB_WAEAVFOutputDevice@@PEAV1@_N@Z
?AddReferencedObjects@UObject@@SAXPEAV1@AEAVFReferenceCollector@@@Z
?InjectDynamicImportsFor@UObject@@SAXAEBVFLinkerLoad@@AEBUFObjectExport@@AEAV?$TArray@UFDynamicImport@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?DeclareConstructClasses@UObject@@SAXAEAV?$TArray@UFTopLevelAssetPath@@V?$TSizedDefaultAllocator@$0CA@@@@@PEBVUClass@@@Z
?AppendToClassSchema@UObject@@SAXAEAUFAppendToClassSchemaContext@@@Z
?DeclareCustomVersions@UObject@@SAXAEAVFArchive@@PEBVUClass@@@Z
?Z_Construct_UClass_UObject@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?GetFullName@UObjectBaseUtility@@QEBA?AVFString@@PEBVUObject@@W4EObjectFullNameFlags@@@Z
?RegisterCompiledInInfo@@YAXPEB_WPEBUFClassRegisterCompiledInInfo@@_KPEBUFStructRegisterCompiledInInfo@@2PEBUFEnumRegisterCompiledInInfo@@2PEBUFPartialRegisterCompiledInInfo@@2@Z
?RegisterCompiledInInfo@@YAXP6APEAVUPackage@@W4ETypeConstructPhase@@@ZPEB_WAEAU?$TRegistrationInfo@VUPackage@@UFPackageReloadVersionInfo@@@@AEBUFPackageReloadVersionInfo@@@Z
?GetStaticEnum@@YAPEAVUEnum@@P6APEAV1@W4ETypeConstructPhase@@@ZPEAVUObject@@PEB_W@Z
?GetStaticStruct@@YAPEAVUScriptStruct@@P6APEAV1@W4ETypeConstructPhase@@@ZPEAVUObject@@PEB_W@Z
?ConstructUClass@UECodeGen_Private@@YAXAEAPEAVUClass@@AEBUFClassParams@1@@Z
?ConstructUPackage@UECodeGen_Private@@YAXAEAPEAVUPackage@@AEBUFPackageParams@1@@Z
?ConstructUScriptStruct@UECodeGen_Private@@YAXAEAPEAVUScriptStruct@@AEBUFStructParams@1@@Z
?ConstructUEnum@UECodeGen_Private@@YAXAEAPEAVUEnum@@AEBUFEnumParams@1@@Z
?AssertIfSubobjectSetupIsNotAllowed@FObjectInitializer@@AEBAXVFName@@@Z
?Add@FOverrides@FObjectInitializer@@QEAAXVFName@@PEBVUClass@@PEBV?$TArrayView@$$CBVFName@@H@@@Z
?Get@FObjectInitializer@@SAAEAV1@XZ
?StaticAllocateObject@@YAPEAVUObject@@PEBVUClass@@PEAV1@VFName@@W4EObjectFlags@@W4EInternalObjectFlags@@_NPEA_NPEAVUPackage@@HUFRemoteObjectId@@PEAVFGCReconstructionGuard@@@Z
?StaticLoadObject@@YAPEAVUObject@@PEAVUClass@@PEAV1@V?$TStringView@_W@@2IPEAVUPackageMap@@_NPEBVFLinkerInstancingContext@@@Z
?GetTransientPackageAsObject@@YAPEAVUObject@@XZ
?MakeObjectRef@Private@CoreUObject@UE@@YA?AUFObjectRef@@UFPackedObjectRef@123@@Z
?Resolve@FObjectRef@@QEBAPEAVUObject@@I@Z
?MarkAsReachable@GC@UE@@YAXPEBVUObject@@@Z
?OnHandleReadInternal@Private@CoreUObject@UE@@YAXPEBVUObject@@@Z
?CheckIsClassChildOf_Internal@@YAXPEBVUClass@@0@Z
?OnHandleReadInternal@Private@CoreUObject@UE@@YAXV?$TArrayView@QEBVUObject@@H@@@Z
```

</details>

<details><summary>UnrealEditor-Engine.dll — 712 functions</summary>

```
?OnAsyncCreatePhysicsStateBegin_GameThread@UPrimitiveComponent@@UEAAXAEAV?$TSet@PEAVUObject@@U?$DefaultKeyFuncs@PEAVUObject@@$0A@@@VFDefaultSetAllocator@@@@@Z
?OnAsyncCreatePhysicsStateEnd_GameThread@UPrimitiveComponent@@UEAAXXZ
?OnAsyncDestroyPhysicsState@UPrimitiveComponent@@UEAA_NAEBVFTimeout@UE@@@Z
?OnAsyncDestroyPhysicsStateBegin_GameThread@UPrimitiveComponent@@UEAAXAEAV?$TSet@PEAVUObject@@U?$DefaultKeyFuncs@PEAVUObject@@$0A@@@VFDefaultSetAllocator@@@@@Z
?OnAsyncDestroyPhysicsStateEnd_GameThread@UPrimitiveComponent@@UEAAXXZ
?CollectBodySetupsWithPhysicsMeshesToCreate@UPrimitiveComponent@@UEBAXAEAV?$TSet@PEAVUBodySetup@@U?$DefaultKeyFuncs@PEAVUBodySetup@@$0A@@@VFDefaultSetAllocator@@@@@Z
?UpdatePhysicsToRBChannels@UPrimitiveComponent@@MEAAXXZ
?PostInitProperties@UPrimitiveComponent@@UEAAXXZ
?PostLoad@UPrimitiveComponent@@UEAAXXZ
?PostDuplicate@UPrimitiveComponent@@UEAAX_N@Z
?GetResourceSizeEx@UPrimitiveComponent@@UEAAXAEAUFResourceSizeEx@@@Z
?BeginDestroy@UPrimitiveComponent@@UEAAXXZ
?FinishDestroy@UPrimitiveComponent@@UEAAXXZ
?IsReadyForFinishDestroy@UPrimitiveComponent@@UEAA_NXZ
?NeedsLoadForClient@UPrimitiveComponent@@UEBA_NXZ
?NeedsLoadForServer@UPrimitiveComponent@@UEBA_NXZ
?CanEditChange@UPrimitiveComponent@@UEBA_NPEBVFProperty@@@Z
?UpdateCollisionProfile@UPrimitiveComponent@@UEAAXXZ
?PostEditImport@UPrimitiveComponent@@UEAAXXZ
?PreSave@UPrimitiveComponent@@UEAAXVFObjectPreSaveContext@@@Z
?OnRenderAssetFirstLodChange@UPrimitiveComponent@@UEAAXPEBVUStreamableRenderAsset@@H@Z
?OnUpdateTransform@UPrimitiveComponent@@UEAAXW4EUpdateTransformFlags@@W4ETeleportType@@@Z
?OnAttachmentChanged@UPrimitiveComponent@@UEAAXXZ
?IsSimulatingPhysics@UPrimitiveComponent@@UEBA_NVFName@@@Z
?MoveComponentImpl@UPrimitiveComponent@@UEAA_NAEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@_NPEAUFHitResult@@W4EMoveComponentFlags@@W4ETeleportType@@@Z
?IsWorldGeometry@UPrimitiveComponent@@UEBA_NXZ
?GetCollisionResponseToChannels@UPrimitiveComponent@@UEBAAEBUFCollisionResponseContainer@@XZ
?GetComponentVelocity@UPrimitiveComponent@@UEBA?AU?$TVector@N@Math@UE@@XZ
?UpdateBounds@UPrimitiveComponent@@UEAAXXZ
?GetNumUncachedStaticLightingInteractions@UPrimitiveComponent@@UEBA?BHXZ
?ShouldDispatchWakeEvents@UPrimitiveComponent@@UEBA_NVFName@@@Z
?InitSweepCollisionParams@UPrimitiveComponent@@UEBAXAEAUFCollisionQueryParams@@AEAUFCollisionResponseParams@@@Z
?GetCollisionShape@UPrimitiveComponent@@UEBA?AUFCollisionShape@@M@Z
?PushSelectionToProxy@UPrimitiveComponent@@UEAAXXZ
?GetPrimitiveStats@UPrimitiveComponent@@UEBAXAEAUFPrimitiveStats@@@Z
?GetHiddenEditorViews@UPrimitiveComponent@@UEBA_KXZ
?SetAllPhysicsAngularVelocityInRadians@UPrimitiveComponent@@UEAAXAEBU?$TVector@N@Math@UE@@_N@Z
?SetAllPhysicsPosition@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@@Z
?SetAllPhysicsRotation@UPrimitiveComponent@@UEAAXAEBU?$TQuat@N@Math@UE@@@Z
?SetAllPhysicsRotation@UPrimitiveComponent@@UEAAXU?$TRotator@N@Math@UE@@@Z
?WakeAllRigidBodies@UPrimitiveComponent@@UEAAXXZ
?SetEnableGravity@UPrimitiveComponent@@UEAAX_N@Z
?IsGravityEnabled@UPrimitiveComponent@@UEBA_NXZ
?SetUpdateKinematicFromSimulation@UPrimitiveComponent@@UEAAX_N@Z
?GetUpdateKinematicFromSimulation@UPrimitiveComponent@@UEBA_NXZ
?GetGyroscopicTorqueEnabled@UPrimitiveComponent@@UEBA_NXZ
?SetLinearDamping@UPrimitiveComponent@@UEAAXM@Z
?GetLinearDamping@UPrimitiveComponent@@UEBAMXZ
?SetAngularDamping@UPrimitiveComponent@@UEAAXM@Z
?GetAngularDamping@UPrimitiveComponent@@UEBAMXZ
?SetMassScale@UPrimitiveComponent@@UEAAXVFName@@M@Z
?GetMassScale@UPrimitiveComponent@@UEBAMVFName@@@Z
?SetAllMassScale@UPrimitiveComponent@@UEAAXM@Z
?SetMassOverrideInKg@UPrimitiveComponent@@UEAAXVFName@@M_N@Z
?GetMass@UPrimitiveComponent@@UEBAMXZ
?GetInertiaTensor@UPrimitiveComponent@@UEBA?AU?$TVector@N@Math@UE@@VFName@@@Z
?ScaleByMomentOfInertia@UPrimitiveComponent@@UEBA?AU?$TVector@N@Math@UE@@U234@VFName@@@Z
?CalculateMass@UPrimitiveComponent@@UEAAMVFName@@@Z
?GetMaxDepenetrationVelocity@UPrimitiveComponent@@UEAAMVFName@@@Z
?SetMaxDepenetrationVelocity@UPrimitiveComponent@@UEAAXVFName@@M@Z
?SetUseCCD@UPrimitiveComponent@@UEAAX_NVFName@@@Z
?SetAllUseCCD@UPrimitiveComponent@@UEAAX_N@Z
?SetUseMACD@UPrimitiveComponent@@UEAAX_NVFName@@@Z
?SetAllUseMACD@UPrimitiveComponent@@UEAAX_N@Z
?SetAllowPartialIslandSleep@UPrimitiveComponent@@UEAAX_NVFName@@@Z
?SetAllAllowPartialIslandSleep@UPrimitiveComponent@@UEAAX_N@Z
?PutAllRigidBodiesToSleep@UPrimitiveComponent@@UEAAXXZ
?IsAnyRigidBodyAwake@UPrimitiveComponent@@UEAA_NXZ
?SetCollisionResponseToChannel@UPrimitiveComponent@@UEAAXW4ECollisionChannel@@W4ECollisionResponse@@@Z
?SetCollisionResponseToAllChannels@UPrimitiveComponent@@UEAAXW4ECollisionResponse@@@Z
?SetCollisionResponseToChannels@UPrimitiveComponent@@UEAAXAEBUFCollisionResponseContainer@@@Z
?OnComponentCollisionSettingsChanged@UPrimitiveComponent@@MEAAX_N@Z
?OnGenerateOverlapEventsChanged@UPrimitiveComponent@@MEAAXXZ
?SetPhysMaterialOverride@UPrimitiveComponent@@UEAAXPEAVUPhysicalMaterial@@@Z
?LineTraceComponent@UPrimitiveComponent@@UEAA_NAEAUFHitResult@@U?$TVector@N@Math@UE@@1W4ECollisionChannel@@AEBUFCollisionQueryParams@@AEBUFCollisionResponseParams@@AEBUFCollisionObjectQueryParams@@@Z
?LineTraceComponent@UPrimitiveComponent@@UEAA_NAEAUFHitResult@@U?$TVector@N@Math@UE@@1AEBUFCollisionQueryParams@@@Z
?SweepComponent@UPrimitiveComponent@@UEAA_NAEAUFHitResult@@U?$TVector@N@Math@UE@@1AEBU?$TQuat@N@45@AEBVFImplicitObject@Chaos@@W4ECollisionChannel@@AEBUFCollisionQueryParams@@AEBUFCollisionResponseParams@@AEBUFCollisionObjectQueryParams@@@Z
?SweepComponent@UPrimitiveComponent@@UEAA_NAEAUFHitResult@@U?$TVector@N@Math@UE@@1AEBU?$TQuat@N@45@AEBUFCollisionShape@@_N@Z
?ComponentOverlapComponentImpl@UPrimitiveComponent@@MEAA_NPEAV1@U?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@AEBUFCollisionQueryParams@@@Z
?ComponentOverlapComponentWithResultImpl@UPrimitiveComponent@@MEBA_NQEBV1@AEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@AEBUFCollisionQueryParams@@AEAV?$TArray@UFOverlapResult@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?OverlapComponent@UPrimitiveComponent@@UEBA_NAEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@AEBUFCollisionShape@@@Z
?OverlapComponentWithResult@UPrimitiveComponent@@UEBA_NAEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@AEBVFImplicitObject@Chaos@@W4ECollisionChannel@@AEBUFCollisionQueryParams@@AEBUFCollisionResponseParams@@AEBUFCollisionObjectQueryParams@@AEAV?$TArray@UFOverlapResult@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?OverlapComponentWithResult@UPrimitiveComponent@@UEBA_NAEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@AEBUFCollisionShape@@AEAV?$TArray@UFOverlapResult@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?ComputePenetration@UPrimitiveComponent@@UEAA_NAEAUFMTDResult@@AEBUFCollisionShape@@AEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@56@@Z
?CanCharacterStepUp@UPrimitiveComponent@@UEBA_NPEAVAPawn@@@Z
?GetNavigationData@UPrimitiveComponent@@UEBAXAEAUFNavigationRelevantData@@@Z
?GetNavigationBounds@UPrimitiveComponent@@UEBA?AU?$TBox@N@Math@UE@@XZ
?GetNavigableGeometryBodySetup@UPrimitiveComponent@@UEAAPEAVUBodySetup@@XZ
?GetNavigableGeometryTransform@UPrimitiveComponent@@UEBA?AU?$TTransform@N@Math@UE@@XZ
?HasCustomNavigableGeometry@UPrimitiveComponent@@UEBA?AW4Type@EHasCustomNavigableGeometry@@XZ
?DoCustomNavigableGeometryExport@UPrimitiveComponent@@UEBA_NAEAUFNavigableGeometryExport@@@Z
?GetPhysicsObjectByName@UPrimitiveComponent@@UEBAPEAUFPhysicsObject@Chaos@@AEBVFName@@@Z
?GetIdFromGTParticle@UPrimitiveComponent@@UEBAHPEAV?$TGeometryParticle@N$02@Chaos@@@Z
?ResolvePhysicsBodyInstanceOwner@UPrimitiveComponent@@UEAAPEAVIPhysicsBodyInstanceOwner@@PEBUFPhysicsObject@Chaos@@@Z
?GetSourceObjectOwner@UPrimitiveComponent@@UEBAPEAVUObject@@XZ
?GetPhysicsMaterialOverride@UPrimitiveComponent@@UEBAPEAVUPhysicalMaterial@@XZ
?GetPhysicsMaterialBase@UPrimitiveComponent@@UEBAPEAVUMaterialInterface@@XZ
?IsPhysicsOwnerMovable@UPrimitiveComponent@@UEBA_NXZ
?IsPhysicsOwnerSimulatingPhysics@UPrimitiveComponent@@UEBA_NXZ
?GetPhysicsOwnerVelocity@UPrimitiveComponent@@UEBA?AU?$TVector@N@Math@UE@@XZ
?GetPhysicsOwnerAttachmentRoot@UPrimitiveComponent@@UEBAPEAVUObject@@XZ
?IsPhysicsObjectWorldGeometry@UPrimitiveComponent@@UEBA_NXZ
?GetPhysicsOwnerSocketTransform@UPrimitiveComponent@@UEBA?AU?$TTransform@N@Math@UE@@VFName@@@Z
?DoesSocketExistOnPhysicsOwner@UPrimitiveComponent@@UEBA_NVFName@@@Z
?HandlePostMigrationPhysicsStateChanged@UPrimitiveComponent@@MEAAXPEAV1@W4EComponentPhysicsStateChange@@@Z
?HasAssetUserDataOfClass@IInterface_AssetUserData@@UEAA_NV?$TSubclassOf@VUAssetUserData@@@@@Z
?AddAssetUserDataOfClass@IInterface_AssetUserData@@UEAA_NV?$TSubclassOf@VUAssetUserData@@@@@Z
?Serialize@UDataAsset@@UEAAXVFStructuredArchiveRecord@@@Z
?Serialize@UDataAsset@@UEAAXAEAVFArchive@@@Z
?GetDisplayNameText@UDataAsset@@UEBA?AVFText@@XZ
?Serialize@UStaticMeshComponent@@UEAAXAEAVFArchive@@@Z
?Serialize@UStaticMeshComponent@@UEAAXVFStructuredArchiveRecord@@@Z
?GetLifetimeReplicatedProps@UStaticMeshComponent@@UEBAXAEAV?$TArray@VFLifetimeProperty@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?ValidateGeneratedRepEnums@UStaticMeshComponent@@UEBAXAEBV?$TArray@UFRepRecord@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?SetStaticMesh@UStaticMeshComponent@@UEAA_NPEAVUStaticMesh@@@Z
?GetNaniteResources@UStaticMeshComponent@@UEBAPEBUFResources@Nanite@@XZ
?HasValidNaniteData@UStaticMeshComponent@@UEBA_NXZ
?UseNaniteOverrideMaterials@UStaticMeshComponent@@UEBA_NXZ
?SetCollisionProfileName@UStaticMeshComponent@@UEAAXVFName@@_N@Z
?BeginDestroy@UStaticMeshComponent@@UEAAXXZ
?ExportCustomProperties@UStaticMeshComponent@@UEAAXAEAVFOutputDevice@@I@Z
?ImportCustomProperties@UStaticMeshComponent@@UEAAXPEB_WPEAVFFeedbackContext@@@Z
?PostInitProperties@UStaticMeshComponent@@UEAAXXZ
?PostReinitProperties@UStaticMeshComponent@@UEAAXXZ
?PostApplyToComponent@UStaticMeshComponent@@UEAAXXZ
?PostEditUndo@UStaticMeshComponent@@UEAAXXZ
?PreEditUndo@UStaticMeshComponent@@UEAAXXZ
?PostEditChangeChainProperty@UStaticMeshComponent@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?CanEditChange@UStaticMeshComponent@@UEBA_NPEBVFProperty@@@Z
?BeginCacheForCookedPlatformData@UStaticMeshComponent@@UEAAXPEBVITargetPlatform@@@Z
?IsCachedCookedPlatformDataLoaded@UStaticMeshComponent@@UEAA_NPEBVITargetPlatform@@@Z
?PostDuplicate@UStaticMeshComponent@@UEAAX_N@Z
?PostEditImport@UStaticMeshComponent@@UEAAXXZ
?InitializeComponent@UStaticMeshComponent@@UEAAXXZ
?UpdateBounds@UStaticMeshComponent@@UEAAXXZ
?PreSave@UStaticMeshComponent@@UEAAXVFObjectPreSaveContext@@@Z
?PostLoad@UStaticMeshComponent@@UEAAXXZ
?IsPostLoadThreadSafe@UStaticMeshComponent@@UEBA_NXZ
?AreNativePropertiesIdenticalTo@UStaticMeshComponent@@UEBA_NPEAVUObject@@@Z
?GetDetailedInfoInternal@UStaticMeshComponent@@UEBA?AVFString@@XZ
?GetMaterialPropertyPath@UStaticMeshComponent@@UEAA_NHAEAPEAVUObject@@AEAVFString@@AEAPEAVFProperty@@@Z
?CalcBounds@UStaticMeshComponent@@UEBA?AU?$TBoxSphereBounds@NN@Math@UE@@AEBU?$TTransform@N@34@@Z
?HasAnySockets@UStaticMeshComponent@@UEBA_NXZ
?QuerySupportedSockets@UStaticMeshComponent@@UEBAXAEAV?$TArray@UFComponentSocketDescription@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetSocketTransform@UStaticMeshComponent@@UEBA?AU?$TTransform@N@Math@UE@@VFName@@W4ERelativeTransformSpace@@@Z
?DoesSocketExist@UStaticMeshComponent@@UEBA_NVFName@@@Z
?ShouldRenderSelected@UStaticMeshComponent@@UEBA_NXZ
?BeginPlay@UStaticMeshComponent@@MEAAXXZ
?RequiresGameThreadEndOfFrameRecreate@UStaticMeshComponent@@MEBA_NXZ
?CreateRenderState_Concurrent@UStaticMeshComponent@@MEAAXPEAVFRegisterComponentContext@@@Z
?OnCreatePhysicsState@UStaticMeshComponent@@MEAAXXZ
?OnDestroyPhysicsState@UStaticMeshComponent@@MEAAXXZ
?OnAsyncDestroyPhysicsStateEnd_GameThread@UStaticMeshComponent@@MEAAXXZ
?ShouldCreatePhysicsState@UStaticMeshComponent@@MEBA_NXZ
?ShouldCreateRenderState@UStaticMeshComponent@@MEBA_NXZ
?ShouldIncrementalPreRegister@UStaticMeshComponent@@MEBA_NPEAVUWorld@@@Z
?GetDefaultMaterialSlotsOverlayMaterial@UStaticMeshComponent@@MEBAXAEAV?$TArray@U?$TObjectPtr@VUMaterialInterface@@@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?AllowsAsyncPhysicsStateCreation@UStaticMeshComponent@@UEBA_NXZ
?AllowsAsyncPhysicsStateDestruction@UStaticMeshComponent@@UEBA_NXZ
?PrecachePSOs@UStaticMeshComponent@@UEAAXXZ
?InvalidateLightingCacheDetailed@UStaticMeshComponent@@UEAAX_N0@Z
?AdditionalStatObject@UStaticMeshComponent@@UEBAPEBVUObject@@XZ
?CheckForErrors@UStaticMeshComponent@@UEAAXXZ
?IsCompiling@UStaticMeshComponent@@UEBA_NXZ
?GetComponentInstanceData@UStaticMeshComponent@@UEBA?AV?$TStructOnScope@UFActorComponentInstanceData@@@@XZ
?IsHLODRelevant@UStaticMeshComponent@@UEBA_NXZ
?ComputeHLODHash@UStaticMeshComponent@@UEBAXAEAVFHLODHashBuilder@@@Z
?GetNumMaterials@UStaticMeshComponent@@UEBAHXZ
?GetStaticLightingInfo@UStaticMeshComponent@@UEAAXAEAUFStaticLightingPrimitiveInfo@@AEBV?$TArray@PEAVULightComponent@@V?$TSizedDefaultAllocator@$0CA@@@@@AEBVFLightingBuildOptions@@@Z
?AddMapBuildDataGUIDs@UStaticMeshComponent@@UEBAXAEAV?$TSet@UFGuid@@U?$DefaultKeyFuncs@UFGuid@@$0A@@@VFDefaultSetAllocator@@@@@Z
?GetEmissiveBoost@UStaticMeshComponent@@UEBAMH@Z
?GetDiffuseBoost@UStaticMeshComponent@@UEBAMH@Z
?GetStaticLightingType@UStaticMeshComponent@@UEBA?AW4ELightMapInteractionType@@XZ
?IsPrecomputedLightingValid@UStaticMeshComponent@@UEBA_NXZ
?GetTextureStreamingTransformScale@UStaticMeshComponent@@UEBAMXZ
?GetMaterialStreamingData@UStaticMeshComponent@@UEBA_NHAEAUFPrimitiveMaterialInfo@@@Z
?BuildTextureStreamingDataImpl@UStaticMeshComponent@@UEAA_NW4ETextureStreamingBuildType@@W4Type@EMaterialQualityLevel@@W43ERHIFeatureLevel@@AEAV?$TSet@UFGuid@@U?$DefaultKeyFuncs@UFGuid@@$0A@@@VFDefaultSetAllocator@@@@AEA_N@Z
?GetStreamableNaniteAsset@UStaticMeshComponent@@UEBAPEAVUStreamableRenderAsset@@XZ
?GetStreamingRenderAssetInfo@UStaticMeshComponent@@UEBAXAEAVFStreamingTextureLevelContext@@AEAV?$TArray@UFStreamingRenderAssetPrimitiveInfo@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetUsedTextures@UStaticMeshComponent@@UEAAXAEAV?$TArray@PEAVUTexture@@V?$TSizedDefaultAllocator@$0CA@@@@@W4Type@EMaterialQualityLevel@@@Z
?RemapActorTextureStreamingBuiltDataToLevel@UStaticMeshComponent@@UEAA_NPEBVUActorTextureStreamingBuildDataComponent@@@Z
?ComputeHashTextureStreamingBuiltData@UStaticMeshComponent@@UEBAIXZ
?GetBodySetup@UStaticMeshComponent@@UEAAPEAVUBodySetup@@XZ
?CanEditSimulatePhysics@UStaticMeshComponent@@UEAA_NXZ
?CreateSceneProxy@UStaticMeshComponent@@UEAAPEAVFPrimitiveSceneProxy@@XZ
?ShouldRecreateProxyOnUpdateTransform@UStaticMeshComponent@@UEBA_NXZ
?UsesOnlyUnlitMaterials@UStaticMeshComponent@@UEBA_NXZ
?GetLightMapResolution@UStaticMeshComponent@@UEBA_NAEAH0@Z
?GetStaticLightMapResolution@UStaticMeshComponent@@UEBAHXZ
?HasValidSettingsForStaticLighting@UStaticMeshComponent@@UEBA_N_N@Z
?GetLightAndShadowMapMemoryUsage@UStaticMeshComponent@@UEBAXAEAH0@Z
?GetUsedMaterials@UStaticMeshComponent@@UEBAXAEAV?$TArray@PEAVUMaterialInterface@@V?$TSizedDefaultAllocator@$0CA@@@@@_N@Z
?GetMaterial@UStaticMeshComponent@@UEBAPEAVUMaterialInterface@@H@Z
?GetEditorMaterial@UStaticMeshComponent@@UEBAPEAVUMaterialInterface@@H@Z
?GetMaterialIndex@UStaticMeshComponent@@UEBAHVFName@@@Z
?GetMaterialFromCollisionFaceIndex@UStaticMeshComponent@@UEBAPEAVUMaterialInterface@@HAEAH@Z
?GetMaterialSlotNames@UStaticMeshComponent@@UEBA?AV?$TArray@VFName@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?IsMaterialSlotNameValid@UStaticMeshComponent@@UEBA_NVFName@@@Z
?DoCustomNavigableGeometryExport@UStaticMeshComponent@@UEBA_NAEAUFNavigableGeometryExport@@@Z
?IsShown@UStaticMeshComponent@@UEBA_NAEBUFEngineShowFlags@@@Z
?PreStaticMeshCompilation@UStaticMeshComponent@@UEAAXXZ
?PostStaticMeshCompilation@UStaticMeshComponent@@UEAAXXZ
?ComponentIsTouchingSelectionBox@UStaticMeshComponent@@UEBA_NAEBU?$TBox@N@Math@UE@@_N1@Z
?ComponentIsTouchingSelectionFrustum@UStaticMeshComponent@@UEBA_NAEBUFConvexVolume@@_N1@Z
?GetPrimitiveStats@UStaticMeshComponent@@UEBAXAEAUFPrimitiveStats@@@Z
?CreateMeshHitProxy@UStaticMeshComponent@@UEBAPEAVHHitProxy@@HH@Z
?RegisterLODStreamingCallback@UStaticMeshComponent@@UEAAX$$QEAV?$TFunction@$$A6AXPEAVIPrimitiveComponent@@PEAVUStreamableRenderAsset@@W4ELODStreamingCallbackResult@@@Z@@0MM@Z
?RegisterLODStreamingCallback@UStaticMeshComponent@@UEAAX$$QEAV?$TFunction@$$A6AXPEAVIPrimitiveComponent@@PEAVUStreamableRenderAsset@@W4ELODStreamingCallbackResult@@@Z@@HM_N@Z
?PrestreamMeshLODs@UStaticMeshComponent@@UEAA_NM@Z
?GetMeshPaintTexture@UStaticMeshComponent@@UEBAPEAVUTexture@@XZ
?SetMeshPaintTexture@UStaticMeshComponent@@UEAAXPEAVUTexture@@@Z
?SetMeshPaintTextureOverride@UStaticMeshComponent@@UEAAXPEAVUTexture@@@Z
?GetMeshPaintTextureCoordinateIndex@UStaticMeshComponent@@UEBAHXZ
?GetNavigationBounds@UStaticMeshComponent@@UEBA?AU?$TBox@N@Math@UE@@XZ
?GetNavigationData@UStaticMeshComponent@@UEBAXAEAUFNavigationRelevantData@@@Z
?UsesTextureLightmaps@UStaticMeshComponent@@UEBA_NHH@Z
?HasLightmapTextureCoordinates@UStaticMeshComponent@@UEBA_NXZ
?GetTextureLightAndShadowMapMemoryUsage@UStaticMeshComponent@@UEBAXHHAEAH0@Z
?GetEstimatedLightMapResolution@UStaticMeshComponent@@UEBAXAEAH0@Z
?GetEstimatedLightAndShadowMapMemoryUsage@UStaticMeshComponent@@UEBA_NAEAH0000AEA_N1@Z
?SupportsDefaultCollision@UStaticMeshComponent@@UEAA_NXZ
?SupportsDitheredLODTransitions@UStaticMeshComponent@@UEAA_NW4EShaderPlatform@@@Z
?SupportsDitheredLODTransitions@UStaticMeshComponent@@UEAA_NW4Type@ERHIFeatureLevel@@@Z
?CollectPSOPrecacheData@UStaticMeshComponent@@MEAAXAEBUFPSOPrecacheParams@@AEAV?$TArray@UFMaterialInterfacePSOPrecacheParams@@V?$TSizedInlineAllocator@$03$0CA@V?$TSizedDefaultAllocator@$0CA@@@@@@@@Z
?CreateStaticMeshSceneProxy@UStaticMeshComponent@@MEAAPEAVFPrimitiveSceneProxy@@AEAUFMaterialAudit@Nanite@@_N@Z
?AllocateStaticLightingMesh@UStaticMeshComponent@@UEAAPEAVFStaticMeshStaticLightingMesh@@HAEBV?$TArray@PEAVULightComponent@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?SetStaticLightingMapping@UStaticMeshComponent@@UEAA_N_NH@Z
?PropagateLightingScenarioChange@UStaticMeshComponent@@UEAAXXZ
?Serialize@AActor@@UEAAXAEAVFArchive@@@Z
?Serialize@AActor@@UEAAXVFStructuredArchiveRecord@@@Z
?ValidateGeneratedRepEnums@AActor@@UEBAXAEBV?$TArray@UFRepRecord@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetLifetimeReplicatedProps@AActor@@UEBAXAEAV?$TArray@VFLifetimeProperty@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetReplicatedCustomConditionState@AActor@@UEBAXAEAVFCustomPropertyConditionState@@@Z
?OnRep_ReplicateMovement@AActor@@UEAAXXZ
?TearOff@AActor@@UEAAXXZ
?HasNetOwner@AActor@@UEBA_NXZ
?HasLocalNetOwner@AActor@@UEBA_NXZ
?OnRep_Owner@AActor@@MEAAXXZ
?SetReplicateMovement@AActor@@UEAAX_N@Z
?OnRep_AttachmentReplication@AActor@@UEAAXXZ
?ReplicateSubobjects@AActor@@UEAA_NPEAVUActorChannel@@PEAVFOutBunch@@PEAUFReplicationFlags@@@Z
?OnSubobjectCreatedFromReplication@AActor@@UEAAXPEAVUObject@@@Z
?OnSubobjectDestroyFromReplication@AActor@@UEAAXPEAVUObject@@@Z
?PreReplication@AActor@@UEAAXAEAVIRepChangedPropertyTracker@@@Z
?PreReplicationForReplay@AActor@@UEAAXAEAVIRepChangedPropertyTracker@@@Z
?RewindForReplay@AActor@@UEAAXXZ
?OnRep_Instigator@AActor@@UEAAXXZ
?IsLockLocation@AActor@@UEBA_NXZ
?OnPlayFromHere@AActor@@UEAAXXZ
?CreateClassActorDesc@AActor@@UEBA?AV?$TUniquePtr@VFWorldPartitionActorDesc@@U?$TDefaultDelete@VFWorldPartitionActorDesc@@@@@@XZ
?GetActorDescProperties@AActor@@UEBAXAEAVFPropertyPairsMap@@@Z
?GetActorDescCustomData@AActor@@UEBAXAEAV?$TSortedMap@VFName@@V?$TArray@EV?$TSizedDefaultAllocator@$0CA@@@@@V?$TSizedDefaultAllocator@$0CA@@@UFNameFastLess@@@@@Z
?EnableInput@AActor@@UEAAXPEAVAPlayerController@@@Z
?CreateInputComponent@AActor@@UEAAXV?$TSubclassOf@VUInputComponent@@@@@Z
?DisableInput@AActor@@UEAAXPEAVAPlayerController@@@Z
?SupportsIncrementalPreUnregisterComponents@AActor@@EEBA_NXZ
?GetActorBounds@AActor@@UEBAX_NAEAU?$TVector@N@Math@UE@@10@Z
?GetVelocity@AActor@@UEBA?AU?$TVector@N@Math@UE@@XZ
?SetActorHiddenInGame@AActor@@UEAAX_N@Z
?K2_DestroyActor@AActor@@UEAAXXZ
?AddTickPrerequisiteActor@AActor@@UEAAXPEAV1@@Z
?AddTickPrerequisiteComponent@AActor@@UEAAXPEAVUActorComponent@@@Z
?RemoveTickPrerequisiteActor@AActor@@UEAAXPEAV1@@Z
?RemoveTickPrerequisiteComponent@AActor@@UEAAXPEAVUActorComponent@@@Z
?BeginPlay@AActor@@MEAAXXZ
?EndPlay@AActor@@MEAAXW4Type@EEndPlayReason@@@Z
?NotifyActorBeginOverlap@AActor@@UEAAXPEAV1@@Z
?NotifyActorEndOverlap@AActor@@UEAAXPEAV1@@Z
?NotifyActorBeginCursorOver@AActor@@UEAAXXZ
?NotifyActorEndCursorOver@AActor@@UEAAXXZ
?NotifyActorOnClicked@AActor@@UEAAXUFKey@@@Z
?NotifyActorOnReleased@AActor@@UEAAXUFKey@@@Z
?NotifyActorOnInputTouchBegin@AActor@@UEAAXW4Type@ETouchIndex@@@Z
?NotifyActorOnInputTouchEnd@AActor@@UEAAXW4Type@ETouchIndex@@@Z
?NotifyActorOnInputTouchEnter@AActor@@UEAAXW4Type@ETouchIndex@@@Z
?NotifyActorOnInputTouchLeave@AActor@@UEAAXW4Type@ETouchIndex@@@Z
?NotifyHit@AActor@@UEAAXPEAVUPrimitiveComponent@@PEAV1@0_NU?$TVector@N@Math@UE@@33AEBUFHitResult@@@Z
?SetLifeSpan@AActor@@UEAAXM@Z
?GetLifeSpan@AActor@@UEBAMXZ
?CheckDefaultSubobjectsInternal@AActor@@UEBA_NXZ
?PostInitProperties@AActor@@UEAAXXZ
?ProcessEvent@AActor@@UEAAXPEAVUFunction@@PEAX@Z
?GetFunctionCallspace@AActor@@UEAAHPEAVUFunction@@PEAUFFrame@@@Z
?CallRemoteFunction@AActor@@UEAA_NPEAVUFunction@@PEAXPEAUFOutParmRec@@PEAUFFrame@@@Z
?PostLoad@AActor@@UEAAXXZ
?PostLoadSubobjects@AActor@@UEAAXPEAUFObjectInstancingGraph@@@Z
?BeginDestroy@AActor@@UEAAXXZ
?Rename@AActor@@UEAA_NPEB_WPEAVUObject@@I@Z
?PostRename@AActor@@UEAAXPEAVUObject@@VFName@@@Z
?CanBeInCluster@AActor@@UEBA_NXZ
?IsEditorOnly@AActor@@UEBA_NXZ
?IsAsset@AActor@@UEBA_NXZ
?PreSaveRoot@AActor@@UEAAXVFObjectPreSaveRootContext@@@Z
?PostSaveRoot@AActor@@UEAAXVFObjectPostSaveRootContext@@@Z
?PreSave@AActor@@UEAAXVFObjectPreSaveContext@@@Z
?Modify@AActor@@UEAA_N_N@Z
?GetActorDescProperties@UPrimitiveComponent@@UEBAXAEAVFPropertyPairsMap@@@Z
?NeedsLoadForTargetPlatform@AActor@@UEBA_NPEBVITargetPlatform@@@Z
?PreEditChange@AActor@@UEAAXPEAVFProperty@@@Z
?PostEditChangeProperty@AActor@@UEAAXAEAUFPropertyChangedEvent@@@Z
?PreEditUndo@AActor@@UEAAXXZ
?PostEditUndo@AActor@@UEAAXV?$TSharedPtr@VITransactionObjectAnnotation@@$00@@@Z
?PostEditUndo@AActor@@UEAAXXZ
?PostTransacted@AActor@@UEAAXAEBVFTransactionObjectEvent@@@Z
?IsSelectedInEditor@AActor@@UEBA_NXZ
?CanDeleteSelectedActor@AActor@@UEBA_NAEAVFText@@@Z
?CanReplaceSelectedActor@AActor@@UEBA_NAEAVFText@@@Z
?SupportsExternalPackaging@AActor@@UEBA_NXZ
?FactoryTransactionAnnotation@AActor@@UEBA?AV?$TSharedPtr@VITransactionObjectAnnotation@@$00@@W4ETransactionAnnotationCreationMode@UObject@@@Z
?PostEditMove@AActor@@UEAAX_N@Z
?PreSaveFromRoot@AActor@@UEAAXVFObjectPreSaveRootContext@@@Z
?PostSaveFromRoot@AActor@@UEAAXVFObjectPostSaveRootContext@@@Z
?CanEditChangeComponent@AActor@@UEBA_NPEBVUActorComponent@@PEBVFProperty@@@Z
?GatherCurrentMovement@AActor@@UEAAXXZ
?GetStreamingBounds@AActor@@UEBAXAEAU?$TBox@N@Math@UE@@0@Z
?ApplyWorldOffset@AActor@@UEAAXAEBU?$TVector@N@Math@UE@@_N@Z
?IsHLODRelevant@AActor@@UEBA_NXZ
?HasHLODRelevantComponents@AActor@@UEBA_NXZ
?GetHLODRelevantComponents@AActor@@UEBA?AV?$TArray@PEAVUActorComponent@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?EditorApplyTranslation@AActor@@UEAAXAEBU?$TVector@N@Math@UE@@_N11@Z
?EditorApplyRotation@AActor@@UEAAXAEBU?$TRotator@N@Math@UE@@_N11@Z
?EditorApplyScale@AActor@@UEAAXAEBU?$TVector@N@Math@UE@@PEBU234@_N22@Z
?EditorApplyMirror@AActor@@UEAAXAEBU?$TVector@N@Math@UE@@0@Z
?EditorGetUnderlyingActors@AActor@@UEBAXAEAV?$TSet@PEAVAActor@@U?$DefaultKeyFuncs@PEAVAActor@@$0A@@@VFDefaultSetAllocator@@@@@Z
?IsHiddenEd@AActor@@UEBA_NXZ
?SetIsTemporarilyHiddenInEditor@AActor@@UEAAX_N@Z
?SetIsHiddenEdLayer@AActor@@UEAA_N_N@Z
?SupportsLayers@AActor@@UEBA_NXZ
?CanChangeIsSpatiallyLoadedFlag@AActor@@UEBA_NXZ
?IsListedInSceneOutliner@AActor@@UEBA_NXZ
?EditorCanAttachTo@AActor@@UEBA_NPEBV1@AEAVFText@@@Z
?EditorCanAttachFrom@AActor@@UEBA_NPEBV1@AEAVFText@@@Z
?GetSceneOutlinerParent@AActor@@UEBAPEAV1@XZ
?GetSceneOutlinerPackage@AActor@@UEBAPEAVUPackage@@XZ
?GetSceneOutlinerItemPackage@AActor@@UEBAPEAVUPackage@@XZ
?GetSceneOutlinerTopParentPackage@AActor@@UEBAPEAVUPackage@@XZ
?EditorReplacedActor@AActor@@UEAAXPEAV1@@Z
?CheckForErrors@AActor@@UEAAXXZ
?CheckForDeprecated@AActor@@UEAAXXZ
?GetDefaultActorLabel@AActor@@UEBA?AVFString@@XZ
?IsActorLabelEditable@AActor@@UEBA_NXZ
?GetReferencedContentObjects@AActor@@UEBA_NAEAV?$TArray@PEAVUObject@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetSoftReferencedContentObjects@AActor@@UEBA_NAEAV?$TArray@UFSoftObjectPath@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetNetPriority@AActor@@UEAAMAEBU?$TVector@N@Math@UE@@0PEAV1@1PEAVUActorChannel@@M_N@Z
?GetReplayPriority@AActor@@UEAAMAEBU?$TVector@N@Math@UE@@0PEAV1@1QEAVUActorChannel@@M@Z
?GetNetDormancy@AActor@@UEAA_NAEBU?$TVector@N@Math@UE@@0PEAV1@1PEAVUActorChannel@@M_N@Z
?SetActorTickEnabled@AActor@@UEAAX_N@Z
?TickActor@AActor@@UEAAXMW4ELevelTick@@AEAUFActorTickFunction@@@Z
?LifeSpanExpired@AActor@@UEAAXXZ
?PreNetReceive@AActor@@UEAAXXZ
?PostNetReceive@AActor@@UEAAXXZ
?PostNetReceiveRole@AActor@@UEAAXXZ
?IsNameStableForNetworking@AActor@@UEBA_NXZ
?IsSupportedForNetworking@AActor@@UEBA_NXZ
?PostNetInit@AActor@@UEAAXXZ
?SendRenderTransform_Concurrent@UPrimitiveComponent@@UEAAXXZ
?PostNetReceiveLocationAndRotation@AActor@@UEAAXXZ
?PostNetReceiveVelocity@AActor@@UEAAXAEBU?$TVector@N@Math@UE@@@Z
?PostNetReceivePhysicState@AActor@@UEAAXXZ
?SetOwner@AActor@@UEAAXPEAV1@@Z
?CheckStillInWorld@AActor@@UEAA_NXZ
?IsDataValid@AActor@@UEBA?AW4EDataValidationResult@@AEAVFDataValidationContext@@@Z
?GetPhysicsVolume@AActor@@UEBAPEAVAPhysicsVolume@@XZ
?Tick@AActor@@UEAAXM@Z
?ShouldTickIfViewportsOnly@AActor@@UEBA_NXZ
?IsNetRelevantFor@AActor@@UEBA_NPEBV1@0AEBU?$TVector@N@Math@UE@@@Z
?IsReplayRelevantFor@AActor@@UEBA_NPEBV1@0AEBU?$TVector@N@Math@UE@@M@Z
?IsRelevancyOwnerFor@AActor@@UEBA_NPEBV1@00@Z
?PreInitializeComponents@AActor@@UEAAXXZ
?PostInitializeComponents@AActor@@UEAAXXZ
?DispatchPhysicsCollisionHit@AActor@@UEAAXAEBUFRigidBodyCollisionInfo@@0AEBUFCollisionImpactData@@@Z
?GetNetOwningPlayer@AActor@@UEAAPEAVUPlayer@@XZ
?GetNetOwningPlayerAnyRole@AActor@@UEAAPEAVUPlayer@@XZ
?GetNetConnection@AActor@@UEBAPEAVUNetConnection@@XZ
?DestroyNetworkActorHandled@AActor@@UEAA_NXZ
?IsSelectionParentOfAttachedActors@AActor@@UEBA_NXZ
?IsSelectionChild@AActor@@UEBA_NXZ
?GetSelectionParent@AActor@@UEBAPEAV1@XZ
?GetRootSelectionParent@AActor@@UEBAPEAV1@XZ
?SupportsSubRootSelection@AActor@@UEBA_NXZ
?PushSelectionToProxies@AActor@@UEAAXXZ
?PushLevelInstanceEditingStateToProxies@AActor@@UEAAX_N@Z
?RegisterAllComponents@AActor@@UEAAXXZ
?PreRegisterAllComponents@AActor@@UEAAXXZ
?UnregisterAllComponents@AActor@@UEAAX_N@Z
?PostUnregisterAllComponents@AActor@@UEAAXXZ
?ReregisterAllComponents@AActor@@UEAAXXZ
?MarkComponentsAsGarbage@AActor@@UEAAX_N@Z
?InvalidateLightingCacheDetailed@AActor@@UEAAX_N@Z
?TeleportTo@AActor@@UEAA_NAEBU?$TVector@N@Math@UE@@AEBU?$TRotator@N@34@_N2@Z
?ClearCrossLevelReferences@AActor@@UEAAXXZ
?IsBasedOnActor@AActor@@UEBA_NPEBV1@@Z
?IsAttachedTo@AActor@@UEBA_NPEBV1@@Z
?RerunConstructionScripts@AActor@@UEAAXXZ
?RegisterReplicationFragments@AActor@@UEAAXAEAVFFragmentRegistrationContext@Net@UE@@W4EFragmentRegistrationFlags@34@@Z
?FillReplicationParams@AActor@@UEAAXAEBUFFillReplicationParamsContext@1@AEAUFActorReplicationParams@@@Z
?OnReplicationStartedForIris@AActor@@MEAAXAEBUFOnReplicationStartedParams@1@@Z
?OnStopReplicationForIris@AActor@@MEAAXAEBUFOnStopReplicationParams@1@@Z
?RegisterActorTickFunctions@AActor@@MEAAX_N@Z
?FellOutOfWorld@AActor@@UEAAXAEBVUDamageType@@@Z
?OutsideWorldBounds@AActor@@UEAAXXZ
?GetComponentsBoundingBox@AActor@@UEBA?AU?$TBox@N@Math@UE@@_N0@Z
?CalculateComponentsBoundingBoxInLocalSpace@AActor@@UEBA?AU?$TBox@N@Math@UE@@_N0@Z
?GetComponentsBoundingCylinder@AActor@@UEBAXAEAM0_N1@Z
?GetSimpleCollisionCylinder@AActor@@UEBAXAEAM0@Z
?IsRootComponentCollisionRegistered@AActor@@UEBA_NXZ
?TornOff@AActor@@UEAAXXZ
?GetComponentsCollisionResponseToChannel@AActor@@UEBA?AW4ECollisionResponse@@W4ECollisionChannel@@@Z
?CanBeBaseForCharacter@AActor@@UEBA_NPEAVAPawn@@@Z
?TakeDamage@AActor@@UEAAMMAEBUFDamageEvent@@PEAVAController@@PEAV1@@Z
?InternalTakeRadialDamage@AActor@@MEAAMMAEBUFRadialDamageEvent@@PEAVAController@@PEAV1@@Z
?InternalTakePointDamage@AActor@@MEAAMMAEBUFPointDamageEvent@@PEAVAController@@PEAV1@@Z
?BecomeViewTarget@AActor@@UEAAXPEAVAPlayerController@@@Z
?EndViewTarget@AActor@@UEAAXPEAVAPlayerController@@@Z
?CalcCamera@AActor@@UEAAXMAEAUFMinimalViewInfo@@@Z
?HasActiveCameraComponent@AActor@@UEBA_N_N@Z
?HasActivePawnControlCameraComponent@AActor@@UEBA_NXZ
?GetHumanReadableName@AActor@@UEBA?AVFString@@XZ
?Reset@AActor@@UEAAXXZ
?GetLastRenderTime@AActor@@UEBAMXZ
?ForceNetRelevant@AActor@@UEAAXXZ
?ForceNetUpdate@AActor@@UEAAXXZ
?PrestreamTextures@AActor@@UEAAXM_NH@Z
?GetActorEyesViewPoint@AActor@@UEBAXAEAU?$TVector@N@Math@UE@@AEAU?$TRotator@N@34@@Z
?GetTargetLocation@AActor@@UEBA?AU?$TVector@N@Math@UE@@PEAV1@@Z
?PostRenderFor@AActor@@UEAAXPEAVAPlayerController@@PEAVUCanvas@@U?$TVector@N@Math@UE@@2@Z
?GetWorld@AActor@@UEBAPEAVUWorld@@XZ
?FindComponentByClass@AActor@@UEBAPEAVUActorComponent@@V?$TSubclassOf@VUActorComponent@@@@@Z
?FindComponentByInterface@AActor@@UEBAPEAVUActorComponent@@V?$TSubclassOf@VUInterface@@@@@Z
?AllowActorComponentToReplicate@AActor@@UEBA?AW4ELifetimeCondition@@PEBVUActorComponent@@@Z
?DisplayDebug@AActor@@UEAAXPEAVUCanvas@@AEBVFDebugDisplayInfo@@AEAM2@Z
?Serialize@UActorComponent@@UEAAXAEAVFArchive@@@Z
?Serialize@UActorComponent@@UEAAXVFStructuredArchiveRecord@@@Z
?ValidateGeneratedRepEnums@UActorComponent@@UEBAXAEBV?$TArray@UFRepRecord@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?CreateClassComponentDesc@UActorComponent@@EEBA?AV?$TUniquePtr@VFWorldPartitionComponentDesc@@U?$TDefaultDelete@VFWorldPartitionComponentDesc@@@@@@XZ
?OnRep_IsActive@UActorComponent@@UEAAXXZ
?Activate@UActorComponent@@UEAAX_N@Z
?Deactivate@UActorComponent@@UEAAXXZ
?SetActive@UActorComponent@@UEAAX_N0@Z
?ToggleActive@UActorComponent@@UEAAXXZ
?SetAutoActivate@UActorComponent@@UEAAX_N@Z
?ReplicateSubobjects@UActorComponent@@UEAA_NPEAVUActorChannel@@PEAVFOutBunch@@PEAUFReplicationFlags@@@Z
?GetComponentClassCanReplicate@UActorComponent@@UEBA_NXZ
?RegisterReplicationFragments@UActorComponent@@UEAAXAEAVFFragmentRegistrationContext@Net@UE@@W4EFragmentRegistrationFlags@34@@Z
?OnReplicationStartedForIris@UActorComponent@@UEAAXAEBUFOnReplicationStartedParams@1@@Z
?OnStopReplicationForIris@UActorComponent@@UEAAXAEBUFOnStopReplicationParams@1@@Z
?ShouldActivate@UActorComponent@@MEBA_NXZ
?ShouldAsyncCreatePhysicsState@UActorComponent@@MEBA_NPEAVUWorld@@@Z
?ShouldAsyncDestroyPhysicsState@UActorComponent@@MEBA_NXZ
?OnPreRegister@UActorComponent@@MEAAXXZ
?OnPreRegistered@UActorComponent@@MEAAXXZ
?OnPreUnregister@UActorComponent@@MEAAXXZ
?OnPreUnregistered@UActorComponent@@MEAAXXZ
?CreateRenderState_Concurrent@UActorComponent@@MEAAXPEAVFRegisterComponentContext@@@Z
?SendRenderTransform_Concurrent@UActorComponent@@MEAAXXZ
?SendRenderDynamicData_Concurrent@UActorComponent@@MEAAXXZ
?SendRenderInstanceData_Concurrent@UActorComponent@@MEAAXXZ
?DestroyRenderState_Concurrent@UActorComponent@@MEAAXXZ
?OnCreatePhysicsState@UActorComponent@@MEAAXXZ
?OnDestroyPhysicsState@UActorComponent@@MEAAXXZ
?RegisterComponentTickFunctions@UActorComponent@@MEAAX_N@Z
?ShouldIncrementalPreRegister@UActorComponent@@UEBA_NPEAVUWorld@@@Z
?ShouldIncrementalPreUnregister@UActorComponent@@UEBA_NXZ
?IsAsyncPhysicsStateCreated@UActorComponent@@UEBA_NXZ
?GetAsyncPhysicsStateObject@UActorComponent@@UEBAPEAVUObject@@XZ
?OnAsyncCreatePhysicsState@UActorComponent@@UEAA_NAEBVFTimeout@UE@@@Z
?OnAsyncCreatePhysicsStateEnd_GameThread@UActorComponent@@UEAAXXZ
?OnAsyncDestroyPhysicsStateBegin_GameThread@UActorComponent@@UEAAXAEAV?$TSet@PEAVUObject@@U?$DefaultKeyFuncs@PEAVUObject@@$0A@@@VFDefaultSetAllocator@@@@@Z
?OnAsyncDestroyPhysicsStateEnd_GameThread@UActorComponent@@UEAAXXZ
?InitializeComponent@UActorComponent@@UEAAXXZ
?ReadyForReplication@UActorComponent@@UEAAXXZ
?BeginPlay@UActorComponent@@UEAAXXZ
?EndPlay@UActorComponent@@UEAAXW4Type@EEndPlayReason@@@Z
?UninitializeComponent@UActorComponent@@UEAAXXZ
?TickComponent@UActorComponent@@UEAAXMW4ELevelTick@@PEAUFActorComponentTickFunction@@@Z
?SetComponentTickEnabled@UActorComponent@@UEAAX_N@Z
?SetComponentTickEnabledAsync@UActorComponent@@UEAAX_N@Z
?IsComponentTickEnabled@UActorComponent@@UEBA_NXZ
?CheckForErrors@UActorComponent@@UEAAXXZ
?RequiresGameThreadEndOfFrameUpdates@UActorComponent@@UEBA_NXZ
?RequiresGameThreadEndOfFrameRecreate@UActorComponent@@UEBA_NXZ
?RequiresPreEndOfFrameSync@UActorComponent@@UEBA_NXZ
?GetReadableName@UActorComponent@@UEBA?AVFString@@XZ
?GetComponentInstanceData@UActorComponent@@UEBA?AV?$TStructOnScope@UFActorComponentInstanceData@@@@XZ
?PostApplyToComponent@UActorComponent@@UEAAXXZ
?BeginDestroy@UActorComponent@@UEAAXXZ
?NeedsLoadForClient@UActorComponent@@UEBA_NXZ
?NeedsLoadForServer@UActorComponent@@UEBA_NXZ
?NeedsLoadForEditorGame@UActorComponent@@UEBA_NXZ
?IsNameStableForNetworking@UActorComponent@@UEBA_NXZ
?IsSupportedForNetworking@UActorComponent@@UEBA_NXZ
?GetLifetimeReplicatedProps@UActorComponent@@UEBAXAEAV?$TArray@VFLifetimeProperty@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetFunctionCallspace@UActorComponent@@UEAAHPEAVUFunction@@PEAUFFrame@@@Z
?CallRemoteFunction@UActorComponent@@UEAA_NPEAVUFunction@@PEAXPEAUFOutParmRec@@PEAUFFrame@@@Z
?PostInitProperties@UActorComponent@@UEAAXXZ
?PostLoad@UActorComponent@@UEAAXXZ
?Rename@UActorComponent@@UEAA_NPEB_WPEAVUObject@@I@Z
?PostRename@UActorComponent@@UEAAXPEAVUObject@@VFName@@@Z
?Modify@UActorComponent@@UEAA_N_N@Z
?CanEditChange@UActorComponent@@UEBA_NPEBVFProperty@@@Z
?PreEditChange@UActorComponent@@UEAAXPEAVFProperty@@@Z
?PostEditChangeChainProperty@UActorComponent@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?PreEditUndo@UActorComponent@@UEAAXXZ
?PostEditUndo@UActorComponent@@UEAAXXZ
?IsSelectedInEditor@UActorComponent@@UEBA_NXZ
?AddAssetUserData@UActorComponent@@UEAAXPEAVUAssetUserData@@@Z
?RemoveUserDataOfClass@UActorComponent@@UEAAXV?$TSubclassOf@VUAssetUserData@@@@@Z
?GetAssetUserDataOfClass@UActorComponent@@UEAAPEAVUAssetUserData@@V?$TSubclassOf@VUAssetUserData@@@@@Z
?GetAssetUserDataArray@UActorComponent@@UEBAPEBV?$TArray@PEAVUAssetUserData@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?DestroyComponent@UActorComponent@@UEAAX_N@Z
?OnComponentCreated@UActorComponent@@UEAAXXZ
?AddTickPrerequisiteActor@UActorComponent@@UEAAXPEAVAActor@@@Z
?AddTickPrerequisiteComponent@UActorComponent@@UEAAXPEAV1@@Z
?RemoveTickPrerequisiteActor@UActorComponent@@UEAAXPEAVAActor@@@Z
?RemoveTickPrerequisiteComponent@UActorComponent@@UEAAXPEAV1@@Z
?Serialize@USceneComponent@@UEAAXAEAVFArchive@@@Z
?Serialize@USceneComponent@@UEAAXVFStructuredArchiveRecord@@@Z
?ValidateGeneratedRepEnums@USceneComponent@@UEBAXAEBV?$TArray@UFRepRecord@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?IsSimulatingPhysics@USceneComponent@@UEBA_NVFName@@@Z
?IsAnySimulatingPhysics@USceneComponent@@UEBA_NXZ
?AttachToComponent@USceneComponent@@UEAA_NPEAV1@AEBUFAttachmentTransformRules@@VFName@@@Z
?DetachFromParent@USceneComponent@@UEAAX_N0@Z
?DetachFromComponent@USceneComponent@@UEAAXAEBUFDetachmentTransformRules@@@Z
?GetSocketTransform@USceneComponent@@UEBA?AU?$TTransform@N@Math@UE@@VFName@@W4ERelativeTransformSpace@@@Z
?GetSocketLocation@USceneComponent@@UEBA?AU?$TVector@N@Math@UE@@VFName@@@Z
?GetSocketRotation@USceneComponent@@UEBA?AU?$TRotator@N@Math@UE@@VFName@@@Z
?GetSocketQuaternion@USceneComponent@@UEBA?AU?$TQuat@N@Math@UE@@VFName@@@Z
?DoesSocketExist@USceneComponent@@UEBA_NVFName@@@Z
?HasAnySockets@USceneComponent@@UEBA_NXZ
?QuerySupportedSockets@USceneComponent@@UEBAXAEAV?$TArray@UFComponentSocketDescription@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetComponentVelocity@USceneComponent@@UEBA?AU?$TVector@N@Math@UE@@XZ
?IsVisible@USceneComponent@@UEBA_NXZ
?GetMaterialPropertyPath@USceneComponent@@UEAA_NHAEAPEAVUObject@@AEAVFString@@AEAPEAVFProperty@@@Z
?OnVisibilityChanged@USceneComponent@@MEAAXXZ
?OnHiddenInGameChanged@USceneComponent@@MEAAXXZ
?GetLifetimeReplicatedProps@USceneComponent@@UEBAXAEAV?$TArray@VFLifetimeProperty@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?OnPreRegister@USceneComponent@@UEAAXXZ
?EndPlay@USceneComponent@@UEAAXW4Type@EEndPlayReason@@@Z
?DestroyComponent@USceneComponent@@UEAAX_N@Z
?ApplyWorldOffset@USceneComponent@@UEAAXAEBU?$TVector@N@Math@UE@@_N@Z
?GetComponentInstanceData@USceneComponent@@UEBA?AV?$TStructOnScope@UFActorComponentInstanceData@@@@XZ
?GetStreamingBounds@USceneComponent@@UEBA?AU?$TBox@N@Math@UE@@XZ
?BeginDestroy@USceneComponent@@UEAAXXZ
?IsPostLoadThreadSafe@USceneComponent@@UEBA_NXZ
?PreNetReceive@USceneComponent@@UEAAXXZ
?PostNetReceive@USceneComponent@@UEAAXXZ
?PostRepNotifies@USceneComponent@@UEAAXXZ
?IsDataValid@USceneComponent@@UEBA?AW4EDataValidationResult@@AEAVFDataValidationContext@@@Z
?NeedsLoadForTargetPlatform@USceneComponent@@UEBA_NPEBVITargetPlatform@@@Z
?PostEditChangeChainProperty@USceneComponent@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?CanEditChange@USceneComponent@@UEBA_NPEBVFProperty@@@Z
?OnUpdateTransform@USceneComponent@@MEAAXW4EUpdateTransformFlags@@W4ETeleportType@@@Z
?UpdateOverlapsImpl@USceneComponent@@MEAA_NPEBV?$TArrayView@$$CBUFOverlapInfo@@H@@_N0@Z
?MoveComponentImpl@USceneComponent@@MEAA_NAEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@34@_NPEAUFHitResult@@W4EMoveComponentFlags@@W4ETeleportType@@@Z
?CalcBounds@USceneComponent@@UEBA?AU?$TBoxSphereBounds@NN@Math@UE@@AEBU?$TTransform@N@34@@Z
?CalcBoundingCylinder@USceneComponent@@UEBAXAEAM0@Z
?UpdateBounds@USceneComponent@@UEAAXXZ
?UpdatePhysicsVolume@USceneComponent@@UEAAX_N@Z
?GetCollisionResponseToChannels@USceneComponent@@UEBAAEBUFCollisionResponseContainer@@XZ
?IsVisibleInEditor@USceneComponent@@UEBA_NXZ
?PostEditComponentMove@USceneComponent@@UEAAX_N@Z
?GetNumUncachedStaticLightingInteractions@USceneComponent@@UEBA?BHXZ
?IsWorldGeometry@USceneComponent@@UEBA_NXZ
?GetCollisionEnabled@USceneComponent@@UEBA?AW4Type@ECollisionEnabled@@XZ
?GetCollisionResponseToChannel@USceneComponent@@UEBA?AW4ECollisionResponse@@W4ECollisionChannel@@@Z
?GetCollisionObjectType@USceneComponent@@UEBA?AW4ECollisionChannel@@XZ
?SetMobility@USceneComponent@@UEAAXW4Type@EComponentMobility@@@Z
?GetPlacementExtent@USceneComponent@@UEBA?AU?$TBoxSphereBounds@NN@Math@UE@@XZ
?Z_Construct_UClass_AActor@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?GetLevel@AActor@@QEBAPEAVULevel@@XZ
?AddOnActorSpawnedHandler@UWorld@@QEBA?AVFDelegateHandle@@AEBV?$TDelegate@$$A6AXPEAVAActor@@@ZUFDefaultDelegateUserPolicy@@@@@Z
?RemoveOnActorSpawnedHandler@UWorld@@QEBAXVFDelegateHandle@@@Z
?GetLevels@UWorld@@QEBAAEBV?$TArray@PEAVULevel@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?GetActiveLevelCollection@UWorld@@QEBAPEBUFLevelCollection@@XZ
?GetWorld@ULevel@@UEBAPEAVUWorld@@XZ
?Z_Construct_UClass_AWorldSettings@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?RegisterComponent@UActorComponent@@QEAAXXZ
?UnregisterComponent@UActorComponent@@QEAAXXZ
?GetRelativeTransform@USceneComponent@@QEBA?AU?$TTransform@N@Math@UE@@XZ
?Destroy@AActor@@QEAA_N_N0@Z
?SetRootComponent@AActor@@QEAA_NPEAVUSceneComponent@@@Z
?RemoveOwnedComponent@AActor@@QEAAXPEAVUActorComponent@@@Z
?RemoveInstanceComponent@AActor@@QEAAXPEAVUActorComponent@@@Z
?IsPlayInEditor@UWorld@@QEBA_NXZ
?GetTimerManager@UWorld@@QEBAAEAVFTimerManager@@XZ
?InternalSetTimer@FTimerManager@@AEAAXAEAUFTimerHandle@@$$QEAUFTimerUnifiedDelegate@@M_NM@Z
?GetPDI@FMeshElementCollector@@QEAAPEAVFPrimitiveDrawInterface@@H@Z
?DrawWireSphere@@YAXPEAVFPrimitiveDrawInterface@@AEBU?$TVector@N@Math@UE@@AEBUFLinearColor@@NHEMM_N@Z
?KeepWorldTransform@FAttachmentTransformRules@@2U1@A
?KeepWorldTransform@FDetachmentTransformRules@@2U1@A
?CheckForErrors@UPrimitiveComponent@@UEAAXXZ
?Z_Construct_UClass_UActorComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?GetActorOwnerNoninline@UActorComponent@@AEBAPEAVAActor@@XZ
?GetWorld_Uncached@UActorComponent@@AEBAPEAVUWorld@@XZ
?OnComponentDestroyed@UPrimitiveComponent@@UEAAX_N@Z
?GetComponentInstanceData@UPrimitiveComponent@@UEBA?AV?$TStructOnScope@UFActorComponentInstanceData@@@@XZ
?HasValidPhysicsState@UPrimitiveComponent@@UEBA_NXZ
?ShouldCreatePhysicsState@UPrimitiveComponent@@UEBA_NXZ
?IsEditorOnly@UPrimitiveComponent@@UEBA_NXZ
?InvalidateLightingCacheDetailed@UPrimitiveComponent@@UEAAX_N0@Z
?OnActorEnableCollisionChanged@UPrimitiveComponent@@UEAAXXZ
?OnDestroyPhysicsState@UPrimitiveComponent@@UEAAXXZ
?OnCreatePhysicsState@UPrimitiveComponent@@UEAAXXZ
?DestroyRenderState_Concurrent@UPrimitiveComponent@@UEAAXXZ
?GetAssetRegistryTags@AActor@@UEBAXVFAssetRegistryTagsContext@@@Z
?PostRename@UPrimitiveComponent@@UEAAXPEAVUObject@@VFName@@@Z
?CreateRenderState_Concurrent@UPrimitiveComponent@@UEAAXPEAVFRegisterComponentContext@@@Z
?SendRenderDebugPhysics@UPrimitiveComponent@@UEAAXPEAVFPrimitiveSceneProxy@@@Z
?ComponentIsTouchingSelectionFrustum@UPrimitiveComponent@@UEBA_NAEBUFConvexVolume@@_N1@Z
?ComponentIsTouchingSelectionBox@UPrimitiveComponent@@UEBA_NAEBU?$TBox@N@Math@UE@@_N1@Z
?IsShown@UPrimitiveComponent@@UEBA_NAEBUFEngineShowFlags@@@Z
?GetComponentTransformFromBodyInstance@UPrimitiveComponent@@UEAA?AU?$TTransform@N@Math@UE@@PEAUFBodyInstance@@@Z
?GetWeldedBodies@UPrimitiveComponent@@UEAAXAEAV?$TArray@PEAUFBodyInstance@@V?$TSizedDefaultAllocator@$0CA@@@@@AEAV?$TArray@VFName@@V?$TSizedDefaultAllocator@$0CA@@@@@_N@Z
?UnWeldChildren@UPrimitiveComponent@@UEAAXXZ
?UnWeldFromParent@UPrimitiveComponent@@UEAAXXZ
?WeldToImplementation@UPrimitiveComponent@@UEAA_NPEAVUSceneComponent@@VFName@@_N2@Z
?WeldTo@UPrimitiveComponent@@UEAAXPEAVUSceneComponent@@VFName@@_N@Z
?ReceiveComponentDamage@UPrimitiveComponent@@UEAAXMAEBUFDamageEvent@@PEAVAController@@PEAVAActor@@@Z
?RecreateInstanceBody@UPrimitiveComponent@@UEAAXH@Z
?GetSquaredDistanceToCollision@UPrimitiveComponent@@UEBA_NAEBU?$TVector@N@Math@UE@@AEAMAEAU234@@Z
?GetBodyInstance@UPrimitiveComponent@@UEBAPEAUFBodyInstance@@VFName@@_NH@Z
?GetRenderMatrix@UPrimitiveComponent@@UEBA?AU?$TMatrix@N@Math@UE@@XZ
?GetUsedTextures@UPrimitiveComponent@@UEAAXAEAV?$TArray@PEAVUTexture@@V?$TSizedDefaultAllocator@$0CA@@@@@W4Type@EMaterialQualityLevel@@@Z
?BuildTextureStreamingDataImpl@UPrimitiveComponent@@UEAA_NW4ETextureStreamingBuildType@@W4Type@EMaterialQualityLevel@@W43ERHIFeatureLevel@@AEAV?$TSet@UFGuid@@U?$DefaultKeyFuncs@UFGuid@@$0A@@@VFDefaultSetAllocator@@@@AEA_N@Z
?GetStreamingRenderAssetInfo@UPrimitiveComponent@@UEBAXAEAVFStreamingTextureLevelContext@@AEAV?$TArray@UFStreamingRenderAssetPrimitiveInfo@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?GetStreamableNaniteAsset@UPrimitiveComponent@@UEBAPEAVUStreamableRenderAsset@@XZ
?GetLightAndShadowMapMemoryUsage@UPrimitiveComponent@@UEBAXAEAH0@Z
?GetLightMapResolution@UPrimitiveComponent@@UEBA_NAEAH0@Z
?UsesOnlyUnlitMaterials@UPrimitiveComponent@@UEBA_NXZ
?ShouldRenderSelected@UPrimitiveComponent@@UEBA_NXZ
?ShouldGenerateAutoLOD@UPrimitiveComponent@@UEBA?B_NH@Z
?SetCollisionObjectType@UPrimitiveComponent@@UEAAXW4ECollisionChannel@@@Z
?SetCollisionProfileName@UPrimitiveComponent@@UEAAXVFName@@_N@Z
?SetCollisionEnabled@UPrimitiveComponent@@UEAAXW4Type@ECollisionEnabled@@@Z
?SetNotifyRigidBodyCollision@UPrimitiveComponent@@UEAAX_N@Z
?WakeRigidBody@UPrimitiveComponent@@UEAAXVFName@@@Z
?SetPhysicsAngularVelocityInRadians@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@_NVFName@@@Z
?SetAllPhysicsLinearVelocity@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@_N@Z
?SetPhysicsLinearVelocity@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@_NVFName@@@Z
?AddTorqueInRadians@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@VFName@@_N@Z
?AddRadialForce@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@MMW4ERadialImpulseFalloff@@_N@Z
?AddForceAtLocationLocal@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@0VFName@@@Z
?AddForceAtLocation@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@0VFName@@@Z
?AddForce@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@VFName@@_N@Z
?AddRadialImpulse@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@MMW4ERadialImpulseFalloff@@_N@Z
?AddVelocityChangeImpulseAtLocation@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@0VFName@@@Z
?AddImpulseAtLocation@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@0VFName@@@Z
?AddAngularImpulseInRadians@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@VFName@@_N@Z
?AddImpulse@UPrimitiveComponent@@UEAAXU?$TVector@N@Math@UE@@VFName@@_N@Z
?SetConstraintMode@UPrimitiveComponent@@UEAAXW4Type@EDOFMode@@@Z
?CanEditSimulatePhysics@UPrimitiveComponent@@UEAA_NXZ
?SetSimulatePhysics@UPrimitiveComponent@@UEAAX_N@Z
?SetWalkableSlopeOverride@UPrimitiveComponent@@UEAAXAEBUFWalkableSlopeOverride@@@Z
?GetWalkableSlopeOverride@UPrimitiveComponent@@UEBAAEBUFWalkableSlopeOverride@@XZ
?GetMaterialFromCollisionFaceIndex@UPrimitiveComponent@@UEBAPEAVUMaterialInterface@@HAEAH@Z
?CreateDynamicMaterialInstance@UPrimitiveComponent@@UEAAPEAVUMaterialInstanceDynamic@@HPEAVUMaterialInterface@@VFName@@@Z
?CreateAndSetMaterialInstanceDynamicFromMaterial@UPrimitiveComponent@@UEAAPEAVUMaterialInstanceDynamic@@HPEAVUMaterialInterface@@@Z
?CreateAndSetMaterialInstanceDynamic@UPrimitiveComponent@@UEAAPEAVUMaterialInstanceDynamic@@H@Z
?SetMaterialByName@UPrimitiveComponent@@UEAAXVFName@@PEAVUMaterialInterface@@@Z
?SetMaterial@UPrimitiveComponent@@UEAAXHPEAVUMaterialInterface@@@Z
?GetMaterialByName@UPrimitiveComponent@@UEBAPEAVUMaterialInterface@@VFName@@@Z
?IsMaterialSlotNameValid@UPrimitiveComponent@@UEBA_NVFName@@@Z
?GetMaterialSlotNames@UPrimitiveComponent@@UEBA?AV?$TArray@VFName@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?GetMaterialIndex@UPrimitiveComponent@@UEBAHVFName@@@Z
?GetMaterial@UPrimitiveComponent@@UEBAPEAVUMaterialInterface@@H@Z
?ComponentOverlapMultiImpl@UPrimitiveComponent@@MEBA_NAEAV?$TArray@UFOverlapResult@@V?$TSizedDefaultAllocator@$0CA@@@@@PEBVUWorld@@AEBU?$TVector@N@Math@UE@@AEBU?$TQuat@N@56@W4ECollisionChannel@@AEBUFComponentQueryParams@@AEBUFCollisionObjectQueryParams@@@Z
?UpdatePhysicsVolume@UPrimitiveComponent@@UEAAX_N@Z
?UpdateOverlapsImpl@UPrimitiveComponent@@UEAA_NPEBV?$TArrayView@$$CBUFOverlapInfo@@H@@_N0@Z
?ShouldComponentIgnoreHitResult@UPrimitiveComponent@@UEAA_NAEBUFHitResult@@W4EMoveComponentFlags@@@Z
?UsePSOPrecacheRenderProxyDelay@UPrimitiveComponent@@MEBA_NXZ
?PrecachePSOs@UPrimitiveComponent@@UEAAXXZ
?SetupPrecachePSOParams@UPrimitiveComponent@@UEAAXAEAUFPSOPrecacheParams@@@Z
?AreAllCollideableDescendantsRelative@UPrimitiveComponent@@MEBA_N_N@Z
?BeginPlay@UPrimitiveComponent@@UEAAXXZ
?ComputeHLODHash@UPrimitiveComponent@@UEBAXAEAVFHLODHashBuilder@@@Z
?Serialize@UPrimitiveComponent@@UEAAXVFStructuredArchiveRecord@@@Z
?Serialize@UPrimitiveComponent@@UEAAXAEAVFArchive@@@Z
?LogMaterialsAndTextures@UMeshComponent@@UEBAXAEAVFOutputDevice@@H@Z
?PrestreamTextures@UMeshComponent@@UEAAXM_NH@Z
?SetTextureForceResidentFlag@UMeshComponent@@UEAAX_N@Z
?GetMaterialRelevance@UMeshComponent@@UEBA?AUFMaterialRelevance@@W4Type@ERHIFeatureLevel@@@Z
?GetMaterialRelevance@UMeshComponent@@UEBA?AUFMaterialRelevance@@W4EShaderPlatform@@@Z
?SetMaterialByName@UMeshComponent@@UEAAXVFName@@PEAVUMaterialInterface@@@Z
?SetMaterial@UMeshComponent@@UEAAXHPEAVUMaterialInterface@@@Z
?GetMaterialByName@UMeshComponent@@UEBAPEAVUMaterialInterface@@VFName@@@Z
?EndPlay@UMeshComponent@@UEAAXW4Type@EEndPlayReason@@@Z
?GetNumOverrideMaterials@UMeshComponent@@UEBAHXZ
?GetMaterials@UMeshComponent@@UEBA?AV?$TArray@PEAVUMaterialInterface@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?IsNavigationRelevant@UStaticMeshComponent@@UEBA_NXZ
?AddReferencedObjects@UStaticMeshComponent@@SAXPEAVUObject@@AEAVFReferenceCollector@@@Z
?GetAllPhysicsObjects@UPrimitiveComponent@@UEBA?AV?$TArray@PEAUFPhysicsObject@Chaos@@V?$TSizedDefaultAllocator@$0CA@@@@@XZ
?GetPhysicsObjectById@UPrimitiveComponent@@UEBAPEAUFPhysicsObject@Chaos@@H@Z
?IsNavigationRelevant@UPrimitiveComponent@@UEBA_NXZ
?GetCollisionObjectType@UPrimitiveComponent@@UEBA?AW4ECollisionChannel@@XZ
?GetCollisionResponseToChannel@UPrimitiveComponent@@UEBA?AW4ECollisionResponse@@W4ECollisionChannel@@@Z
?GetCollisionEnabled@UPrimitiveComponent@@UEBA?AW4Type@ECollisionEnabled@@XZ
?HasStaticLighting@UPrimitiveComponent@@QEBA_NXZ
?MarkRenderStateDirty@UActorComponent@@QEAAXXZ
?AddReferencedObjects@UActorComponent@@SAXPEAVUObject@@AEAVFReferenceCollector@@@Z
?ReceiveAsyncPhysicsTick@UActorComponent@@QEAAXMM@Z
?GetLocalBounds@USceneComponent@@QEBA?AU?$TBoxSphereBounds@NN@Math@UE@@XZ
?AddReferencedObjects@USceneComponent@@SAXPEAVUObject@@AEAVFReferenceCollector@@@Z
?UpdateComponentToWorldWithParent@USceneComponent@@AEAAXPEAV1@VFName@@W4EUpdateTransformFlags@@AEBU?$TQuat@N@Math@UE@@W4ETeleportType@@@Z
?Z_Construct_UClass_UChildActorComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?SetActorLocation@AActor@@QEAA_NAEBU?$TVector@N@Math@UE@@_NPEAUFHitResult@@W4ETeleportType@@@Z
?ReceiveAsyncPhysicsTick@AActor@@QEAAXMM@Z
?AddReferencedObjects@AActor@@SAXPEAVUObject@@AEAVFReferenceCollector@@@Z
?GetFolderPath@AActor@@QEBA?AVFName@@XZ
?SetFolderPath@AActor@@QEAAXAEBVFName@@@Z
?Z_Construct_UClass_UTexture2D@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?RegisterCustomShowFlag@FEngineShowFlags@@SA?AW4ECustomShowFlag@1@PEB_W_NW4EShowFlagGroup@@VFText@@@Z
?OnRep_ReplicatedMovement@AActor@@UEAAXXZ
```

</details>

<details><summary>UnrealEditor-UnrealEd.dll — 2 functions</summary>

```
?GEditor@@3PEAVUEditorEngine@@EA
?GetEditorWorldContext@UEditorEngine@@QEAAAEAUFWorldContext@@_N@Z
```

</details>

<details><summary>UnrealEditor-DeveloperSettings.dll — 11 functions</summary>

```
?PostEditChangeProperty@UDeveloperSettings@@UEAAXAEAUFPropertyChangedEvent@@@Z
?GetSectionDescription@UDeveloperSettings@@UEBA?AVFText@@XZ
?GetSectionText@UDeveloperSettings@@UEBA?AVFText@@XZ
?GetSectionName@UDeveloperSettings@@UEBA?AVFName@@XZ
?GetCategoryName@UDeveloperSettings@@UEBA?AVFName@@XZ
?GetContainerName@UDeveloperSettings@@UEBA?AVFName@@XZ
??0UDeveloperSettings@@QEAA@AEBVFObjectInitializer@@@Z
??1UDeveloperSettings@@UEAA@XZ
??0UDeveloperSettings@@QEAA@AEAVFVTableHelper@@@Z
?Z_Construct_UClass_UDeveloperSettings@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?GetCustomSettingsWidget@UDeveloperSettings@@UEBA?AV?$TSharedPtr@VSWidget@@$00@@XZ
```

</details>

<details><summary>UnrealEditor-Core.dll — 90 functions</summary>

```
?GetAllocSizeExternal@FMemory@@SA_KPEAX@Z
?OnInvalidArrayNum@Private@Core@UE@@YAX_K@Z
?ResizeAllocation@ForAnyElementType@?$TSizedHeapAllocator@$0CA@UFMemory@@@@QEAAXHH_KI@Z
??0FString@@QEAA@PEBD@Z
??0FString@@QEAA@PEB_W@Z
?FromValidEName@FNameEntryId@@CA?AU1@W4EName@@@Z
?ToString@FName@@QEBA?AVFString@@XZ
??0FName@@QEAA@PEB_W@Z
??0FName@@QEAA@PEBD@Z
??0FName@@QEAA@HPEB_WW4EFindName@@@Z
??0FNameDebugVisualizer@@QEAA@UFClangKeepDebugInfo@@@Z
?GetBlocks@FNameDebugVisualizer@@QEAAPEAPEAEXZ
??0FLogCategoryBase@@QEAA@PEB_WW4Type@ELogVerbosity@@1@Z
??1FLogCategoryBase@@QEAA@XZ
?NewGuid@FGuid@@SA?AU1@XZ
??0FMemScope@@QEAA@W4ELLMTag@@_N@Z
??1FMemScope@@QEAA@XZ
?FindLLMMemoryPool@FPoolScope@LLMInterop@RED@@SA?AUFMemoryPoolHandle@@U4@@Z
?Init@FPoolScope@LLMInterop@RED@@AEAAXW4ELLMTag@@@Z
?Deinit@FPoolScope@LLMInterop@RED@@AEAAXXZ
?Construct@FLowLevelMemTracker@@SAAEAV1@XZ
??0FLLMScope@@QEAA@W4ELLMTag@@_NW4ELLMTagSet@@W4ELLMTracker@@1@Z
??1FLLMScope@@QEAA@XZ
?FromHex@FColor@@SA?AU1@AEBVFString@@@Z
?Add@FStatsTrace@@SAXAEBVFName@@_J@Z
?Set@FStatsTrace@@SAXAEBVFName@@_J@Z
?Get@FThreadStatsPool@@SAAEAU1@XZ
?GetFromPool@FThreadStatsPool@@QEAAPEAVFThreadStats@@XZ
?Flush@FThreadStats@@QEAAX_N0@Z
?FlushRawStats@FThreadStats@@QEAAX_N0@Z
?DoSetup@FThreadSafeStaticStatBase@@IEBAPEBUTStatIdData@@VFName@@PEB_WPEBD21_N3W4Type@EStatDataType@@33W4EMemoryCounterRegion@FWindowsPlatformMemory@@@Z
?AsCultureInvariant@FText@@SA?AV1@PEB_W@Z
??$BasicLog@$01_W@Private@Logging@UE@@YAXAEBU?$TStaticBasicLogRecord@_W@012@PEBUFLogCategoryBase@@ZZ
?Quaternion@?$TRotator@N@Math@UE@@QEBA?AU?$TQuat@N@23@XZ
?Reset@FUtf8String@@QEAAXH@Z
?MallocExternal@FMemory@@SAPEAX_KI@Z
??1FModuleInitializerEntry@@QEAA@XZ
?GMalloc@Private@UE@@3PEAVFMalloc@@EA
?PrimarySlot@WindowsPlatformTLS_Private@@3IA
?LogTemp@@3UFLogCategoryLogTemp@@A
?PRIVATE_GIsRunningCommandlet@@3_NA
?IsInAsyncLoadingThread@@3P6A_NXZEA
?TrackerInstance@FLowLevelMemTracker@@0PEAV1@EA
?EnabledState@FLowLevelMemTracker@@0W4EEnabled@LLMPrivate@UE@@A
?Red@FLinearColor@@2U1@B
?White@FColor@@2U1@B
?TlsSlot@FThreadStats@@0IA
?bPrimaryEnable@FThreadStats@@0_NA
?bPrimaryDisableForever@FThreadStats@@0_NA
?bIsRawStatsActive@FThreadStats@@0_NA
?ZeroRotator@?$TRotator@N@Math@UE@@2U123@B
?Identity@?$TQuat@N@Math@UE@@2U123@B
?OnInvalidSetNum@Private@Core@UE@@YAX_K@Z
?ResizeAllocation@ForAnyElementType@?$TSizedHeapAllocator@$0CA@UFMemory@@@@QEAAXHH_K@Z
?IsInGameThread@@YA_NXZ
?GenerateNewID@FDelegateHandle@@CA_KXZ
?StackWalkAndDump@FWindowsPlatformStackWalk@@SAXPEAD_KHPEAX@Z
?ThreadStackWalkAndDump@FWindowsPlatformStackWalk@@SAXPEAD_KHI@Z
?GetDestructionSentinelStackTls@FMRSWRecursiveAccessDetector@@CAAEAV?$TArray@PEAUFDestructionSentinel@FMRSWRecursiveAccessDetector@@V?$TSizedInlineAllocator@$03$0CA@V?$TSizedDefaultAllocator@$0CA@@@@@@@XZ
?GetReadersTls@FMRSWRecursiveAccessDetector@@CAAEAV?$TArray@UFReaderNum@FMRSWRecursiveAccessDetector@@V?$TSizedInlineAllocator@$03$0CA@V?$TSizedDefaultAllocator@$0CA@@@@@@@XZ
?DelegateAllocate@Private@Core@UE@@YAPEAX_KAEAUFDelegateAllocation@@@Z
??$ClosestPointOnLine@N@FMath@@SA?AU?$TVector@N@Math@UE@@AEBU123@00@Z
?bSuppressCheckFailure@FMTAccessDetectorOptions@Private@UE@@0_NA
?Identity@?$TTransform@N@Math@UE@@2U123@B
?HandleAtomicsFailure@FWindowsPlatformAtomics@@KAXPEB_WZZ
?OutputBeginEvent@FCpuProfilerTrace@@SAXI@Z
?OutputEndEvent@FCpuProfilerTrace@@SAXXZ
?GetOrCreateSpecId@FCpuProfilerTrace@@SAXAEAIPEBD1I@Z
?GetModulePtr_Internal@FModuleManager@@CAPEAVIModuleInterface@@VFName@@@Z
??$BasicLog@$03_W@Private@Logging@UE@@YAXAEBU?$TStaticBasicLogRecord@_W@012@PEBUFLogCategoryBase@@ZZ
?CpuChannel@@3AEAVFChannel@Trace@UE@@EA
?QuantizeSize@FMemory@@SA_K_KI@Z
?Free@FMemory@@SAXPEAX@Z
?FreeExternal@FMemory@@SAXPEAX@Z
?Realloc@FMemory@@SAPEAXPEAX_KIUFMemoryPoolHandle@@@Z
?Malloc@FMemory@@SAPEAX_KIUFMemoryPoolHandle@@@Z
?DoGamethreadHook@@YAXH@Z
?Instance@FPoolFMemoryUncategorized@@SA?AUFMemoryPoolHandle@@XZ
?MemoryPoolsAreEnabled@MemoryPoolInternal@@YA_NXZ
?Instance@FPoolUObjectUncategorized@@SA?AUFMemoryPoolHandle@@XZ
?GetExtraData@FMemoryPoolDebugVisualizer@@SAPEAPEAUFMemoryPoolExtraData@MemoryPoolInternal@@XZ
?EnsureFailed@Private@Assert@UE@@YA_NAEAU?$atomic@E@std@@PEBUFStaticEnsureRecord@123@ZZ
?CheckEnsureFailed@Private@Assert@UE@@YA_N_NAEBU?$atomic@E@std@@@Z
?CheckVerifyFailedImpl2@FDebug@@SA_NPEBD0HPEB_WZZ
?ZeroVector@?$TVector@N@Math@UE@@2U123@B
?GCoreDebuggingState@@3PEAUFVisualizerDebuggingState@Core@UE@@EA
?GCoreObjectHandlePackageDebug@@3PEAUFObjectHandlePackageDebugData@Private@CoreUObject@UE@@EA
?GCoreComplexObjectPathDebug@@3PEAUFStoredObjectPathDebug@Private@CoreUObject@UE@@EA
??0FModuleInitializerEntry@@QEAA@PEB_WP6APEAVIModuleInterface@@XZ0@Z
?GCoreObjectArrayForDebugVisualizers@@3PEAVFChunkedFixedUObjectArray@@EA
```

</details>

<details><summary>UnrealEditor-RedUniversalSpline.dll — 64 functions</summary>

```
?GetLastControlPoint@URedUniversalSpline@@QEAAPEAVURedUniversalSplineControlPoint@@XZ
?GetFirstControlPoint@URedUniversalSpline@@QEAAPEAVURedUniversalSplineControlPoint@@XZ
?GetNextControlPointIndex@URedUniversalSpline@@QEBAHH@Z
?EvaluateRightVector@URedUniversalSpline@@QEBA?AU?$TVector@N@Math@UE@@HMAEBU234@@Z
?EvaluateTangent@URedUniversalSpline@@QEBA?AU?$TVector@N@Math@UE@@HM_N0@Z
?EvaluatePoint@URedUniversalSpline@@QEBA?AU?$TVector@N@Math@UE@@M_N@Z
?GetSplineSegmentIndexAtDistance@URedUniversalSpline@@QEBAHM@Z
?GetSplineSegmentNum@URedUniversalSpline@@QEBAHXZ
?GetPreLastControlPoint@URedUniversalSpline@@QEAAPEAVURedUniversalSplineControlPoint@@XZ
?GetSecondControlPoint@URedUniversalSpline@@QEAAPEAVURedUniversalSplineControlPoint@@XZ
?GetSplineSegmentControlPoints@URedUniversalSpline@@QEBA_NHAEAPEAVURedUniversalSplineControlPoint@@0@Z
?GetSplineDistanceClosestToLocation@URedUniversalSpline@@QEAAMAEBU?$TVector@N@Math@UE@@_N@Z
?IsLocationOnStraightSegment@URedUniversalSpline@@QEBA_NAEBU?$TVector@N@Math@UE@@MAEAH11AEAM_NH@Z
?GetLeftSideLocation@URedUniversalSplineControlPoint@@QEBA?AU?$TVector@N@Math@UE@@_N@Z
?GetRightSideLocation@URedUniversalSplineControlPoint@@QEBA?AU?$TVector@N@Math@UE@@_N@Z
?IsLast@URedUniversalSplineControlPoint@@QEAA_NXZ
?IsFirst@URedUniversalSplineControlPoint@@QEAA_NXZ
?GetSpline@URedUniversalSplineControlPoint@@QEAAPEAVURedUniversalSpline@@XZ
?GetHandle@URedUniversalSplineControlPoint@@QEBA?AU?$TVector@N@Math@UE@@W4ERedUniversalSplineHandleType@@_N@Z
?GetRightVector@URedUniversalSplineControlPoint@@QEBA?AU?$TVector@N@Math@UE@@_N@Z
?GetTangent@URedUniversalSplineControlPoint@@QEBA?AU?$TVector@N@Math@UE@@_N@Z
?SetLocation@URedUniversalSplineControlPoint@@QEAAXAEBU?$TVector@N@Math@UE@@_N@Z
?GetLocation@URedUniversalSplineControlPoint@@QEBA?AU?$TVector@N@Math@UE@@_N@Z
?Update@URedUniversalSpline@@QEAAXXZ
?MoveHandle@URedUniversalSplineControlPoint@@UEAAXW4ERedUniversalSplineHandleType@@AEBU?$TVector@N@Math@UE@@_N2@Z
?SetHandle@URedUniversalSplineControlPoint@@UEAAXW4ERedUniversalSplineHandleType@@AEBU?$TVector@N@Math@UE@@_N2@Z
?PostEditChangeChainProperty@URedUniversalSplineControlPoint@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?PostEditChangeProperty@URedUniversalSplineControlPoint@@UEAAXAEAUFPropertyChangedEvent@@@Z
?InitializeControlPoint@URedUniversalSpline@@MEAAXPEAVURedUniversalSplineControlPoint@@HMU?$TVector@N@Math@UE@@1@Z
?InitializeControlPoint@URedUniversalSpline@@MEAAXPEAVURedUniversalSplineControlPoint@@H_NU?$TVector@N@Math@UE@@22@Z
?GenerateSamples@URedUniversalSpline@@MEAAHM@Z
?DrawWideVisualization@URedUniversalSpline@@UEAAXPEAVFMeshElementCollector@@PEAVFPrimitiveDrawInterface@@HW4ESceneDepthPriorityGroup@@_NU?$TVector@N@Math@UE@@PEAVHHitProxy@@V?$TFunctionRef@$$A6A?AUFLinearColor@@H@Z@@H@Z
?DrawVisualization@URedUniversalSpline@@UEAAXPEAVFMeshElementCollector@@PEAVFPrimitiveDrawInterface@@H_NW4ESceneDepthPriorityGroup@@2U?$TVector@N@Math@UE@@PEAVHHitProxy@@V?$TFunctionRef@$$A6A?AUFLinearColor@@H@Z@@H2@Z
?AppendControlPoint@URedUniversalSpline@@UEAAH_NU?$TVector@N@Math@UE@@0@Z
?InsertControlPoint@URedUniversalSpline@@UEAAHM@Z
?CreateGizmoFillMeshBuilder@URedUniversalSpline@@UEAA_NAEAVFDynamicMeshBuilder@@@Z
?GenerateLoopFillInfoMeshData@URedUniversalSpline@@UEAAXXZ
?GenerateRibbonInfoMeshData@URedUniversalSpline@@UEAAXXZ
?UpdateInfoMeshData@URedUniversalSpline@@UEAAXXZ
?CalculateBounds@URedUniversalSpline@@UEAA?AU?$TBoxSphereBounds@NN@Math@UE@@_N0@Z
?PostEditUndo@URedUniversalSpline@@UEAAXXZ
?PostEditChangeChainProperty@URedUniversalSpline@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?PostEditChangeProperty@URedUniversalSpline@@UEAAXAEAUFPropertyChangedEvent@@@Z
?PostLoad@URedUniversalSpline@@UEAAXXZ
?ShouldGenerateLoopFillInfoMesh@URedUniversalSpline@@UEBA_NXZ
?ShouldGenerateRibbonInfoMesh@URedUniversalSpline@@UEBA_NXZ
?GetFalloffWidthHandleGizmoSize@URedUniversalSpline@@UEBAMXZ
?GetFalloffWidthHandleGizmoColor@URedUniversalSpline@@UEBA?AUFLinearColor@@XZ
?GetWidthHandleGizmoColor@URedUniversalSpline@@UEBA?AUFLinearColor@@XZ
?GetWidthHandleGizmoSize@URedUniversalSpline@@UEBAMXZ
?GetSideSplineGizmoColor@URedUniversalSpline@@UEBA?AUFLinearColor@@XZ
?CanSnapControlPoints@URedUniversalSpline@@UEBA_NXZ
?GetFalloffSplineGizmoColor@URedUniversalSpline@@UEBA?AUFLinearColor@@XZ
?Z_Construct_UClass_URedUniversalSplineControlPoint@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedUniversalSplineControlPoint@@SAPEAVUClass@@XZ
??0URedUniversalSplineControlPoint@@QEAA@AEAVFVTableHelper@@@Z
??1URedUniversalSplineControlPoint@@UEAA@XZ
??0URedUniversalSplineControlPoint@@QEAA@XZ
?Move@URedUniversalSplineControlPoint@@UEAAXAEBU?$TVector@N@Math@UE@@@Z
?CanAddControlPoints@URedUniversalSpline@@UEBA_NXZ
?CanDeleteControlPoints@URedUniversalSpline@@UEBA_NXZ
?CanAddControlPointsFromSide@URedUniversalSpline@@UEBA_NXZ
?CanAddControlPointsFromEnd@URedUniversalSpline@@UEBA_NXZ
?CanInvertDirection@URedUniversalSpline@@UEBA_NXZ
```

</details>

<details><summary>UnrealEditor-RedSplineTool.dll — 139 functions</summary>

```
?IsDragAndDropGhostActor@RedSplineToolUtils@@YA_NPEBVAActor@@@Z
?SetActorIcon@RedSplineToolUtils@@YAXPEBVAActor@@PEAVUTexture2D@@@Z
?IsCircleIntersectingRectangle@RedSplineToolUtils@@YA_NAEBU?$TBox@N@Math@UE@@U?$TVector2@N@34@M@Z
?GetDistanceOnLine@RedSplineToolUtils@@YAMAEBU?$TVector@N@Math@UE@@00@Z
?Settings@FRedSplineToolModule@@2PEBVURedSplineToolSettings@@EB
?ShouldShowFalloffLines@URedSplineToolSpline@@UEBA_NXZ
?ShouldShowLeftFalloffWidthHandles@URedSplineToolSpline@@UEBA_NXZ
?ShouldShowWidthHandles@URedSplineToolSpline@@UEBA_NXZ
?ShouldShowInfoMesh@URedSplineToolSpline@@UEBA_NXZ
?GetControlPointGizmoColor@URedSplineToolSpline@@UEBA?AUFLinearColor@@XZ
?GetSplineGizmoColor@URedSplineToolSpline@@UEBA?AUFLinearColor@@XZ
?Update@URedSplineToolLandscapeComponent@@UEAAXXZ
?PostEditChangeProperty@URedSplineToolLandscapeComponent@@UEAAXAEAUFPropertyChangedEvent@@@Z
?OnComponentDestroyed@URedSplineToolLandscapeComponent@@UEAAX_N@Z
?SetCustomPrimitiveData@URedSplineToolMeshComponent@@MEAAX_N@Z
?RemovePropertiesFromControlPoints@URedSplineToolMeshComponent@@MEAAXPEAVURedUniversalSpline@@@Z
?AddMissingPropertiesToControlPoints@URedSplineToolMeshComponent@@MEAAXPEAVURedUniversalSpline@@@Z
?DrawVisualization@URedSplineToolMeshComponent@@MEAAXPEAVFPrimitiveDrawInterface@@@Z
?Update@URedSplineToolMeshComponent@@UEAAXXZ
?OnComponentDestroyed@URedSplineToolMeshComponent@@UEAAX_N@Z
?PostEditChangeProperty@URedSplineToolMeshComponent@@UEAAXAEAUFPropertyChangedEvent@@@Z
?PreEditChange@URedSplineToolMeshComponent@@UEAAXPEAVFProperty@@@Z
?OnComponentCreated@URedSplineToolMeshComponent@@UEAAXXZ
?PostEditImport@ARedSplineTool@@UEAAXXZ
?IsSelectable@ARedSplineTool@@UEBA_NXZ
?CanEditChange@ARedSplineTool@@UEBA_NPEBVFProperty@@@Z
?GetNewPresetSaveFolder@ARedSplineTool@@UEBA?AVFString@@XZ
?GetPresetClass@ARedSplineTool@@UEBAPEAVUClass@@XZ
?GetDefaultPreset@ARedSplineTool@@UEAA?AU?$TSoftObjectPtr@VURedSplineToolPreset@@@@XZ
?Destroyed@ARedSplineTool@@UEAAXXZ
?PostRegisterAllComponents@ARedSplineTool@@UEAAXXZ
?OnConstruction@ARedSplineTool@@UEAAXAEBU?$TTransform@N@Math@UE@@@Z
?RemovePropertiesFromControlPoints@URedSplineToolRVTDecalComponent@@MEAAXPEAVURedUniversalSpline@@@Z
?AddMissingPropertiesToControlPoints@URedSplineToolRVTDecalComponent@@MEAAXPEAVURedUniversalSpline@@@Z
?DrawVisualization@URedSplineToolRVTDecalComponent@@MEAAXPEAVFPrimitiveDrawInterface@@@Z
?CreateSplineMeshComponents@URedSplineToolRVTDecalComponent@@MEAA?AV?$TArray@PEAVUSplineMeshComponent@@V?$TSizedDefaultAllocator@$0CA@@@@@PEAVURedUniversalSpline@@AEBU?$TVector@N@Math@UE@@@Z
?SetupRVTSplineMeshComponent@URedSplineToolRVTDecalComponent@@MEAAXV?$TArray@PEAVUSplineMeshComponent@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?Update@URedSplineToolRVTDecalComponent@@UEAAXXZ
?OnComponentDestroyed@URedSplineToolRVTDecalComponent@@UEAAX_N@Z
?OnComponentCreated@URedSplineToolRVTDecalComponent@@UEAAXXZ
?OnRegister@URedSplineToolRVTDecalComponent@@UEAAXXZ
?OnSplineControlPointsChanged@URedSplineToolComponent@@UEAAXPEAVURedUniversalSplineControlPoint@@_N@Z
?CreateMainSpline@URedSplineToolComponent@@UEAAXXZ
?SnapToLandscape@URedSplineToolComponent@@UEAAXAEAV?$TArray@PEAVURedUniversalSplineControlPoint@@V?$TSizedDefaultAllocator@$0CA@@@@@@Z
?OnApplyPreset@URedSplineToolComponent@@UEAAXPEAVURedSplineToolPreset@@@Z
?Update@URedSplineToolComponent@@UEAAX_N@Z
?DrawFullVisualization@URedSplineToolComponent@@UEBAXPEAVFMeshElementCollector@@PEBVFSceneView@@H_N2PEAVHHitProxy@@H@Z
?DrawVisualization@URedSplineToolComponent@@UEBAXPEAVFMeshElementCollector@@PEBVFSceneView@@H_N2PEAVHHitProxy@@@Z
?CalculateSimplificationDistance@URedSplineToolComponent@@UEBAMAEBU?$TVector@N@Math@UE@@@Z
?Z_Construct_UClass_ARedSplineTool@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@ARedSplineTool@@SAPEAVUClass@@XZ
??0ARedSplineTool@@QEAA@AEBVFObjectInitializer@@@Z
??0ARedSplineTool@@QEAA@AEAVFVTableHelper@@@Z
??1ARedSplineTool@@UEAA@XZ
?PostActorCreated@ARedSplineTool@@UEAAXXZ
?Z_Construct_UClass_URedSplineToolSpline@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolSpline@@SAPEAVUClass@@XZ
??0URedSplineToolSpline@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolSpline@@UEAA@XZ
??0URedSplineToolSpline@@QEAA@XZ
?Z_Construct_UClass_URedSplineToolPreset@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolPreset@@SAPEAVUClass@@XZ
??0URedSplineToolPreset@@QEAA@AEBVFObjectInitializer@@@Z
??0URedSplineToolPreset@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolPreset@@UEAA@XZ
?IsEditorInLevelInstanceEditingMode@RedSplineToolUtils@@YA_NXZ
?Z_Construct_UClass_URedSplineToolComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolComponent@@SAPEAVUClass@@XZ
??0URedSplineToolComponent@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolComponent@@UEAA@XZ
??0URedSplineToolComponent@@QEAA@AEBVFObjectInitializer@@@Z
?PostEditChangeProperty@URedSplineToolComponent@@UEAAXAEAUFPropertyChangedEvent@@@Z
?Z_Construct_UClass_URedSplineToolLandscapeComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolLandscapeComponent@@SAPEAVUClass@@XZ
?_getUObject@URedSplineToolLandscapeComponent@@UEBAPEAVUObject@@XZ
??0URedSplineToolLandscapeComponent@@QEAA@AEBVFObjectInitializer@@@Z
??0URedSplineToolLandscapeComponent@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolLandscapeComponent@@UEAA@XZ
?OnRegister@URedSplineToolLandscapeComponent@@UEAAXXZ
?OnUnregister@URedSplineToolLandscapeComponent@@UEAAXXZ
?Z_Construct_UClass_URedSplineToolMeshComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolMeshComponent@@SAPEAVUClass@@XZ
?_getUObject@URedSplineToolMeshComponent@@UEBAPEAVUObject@@XZ
??0URedSplineToolMeshComponent@@QEAA@AEBVFObjectInitializer@@@Z
??0URedSplineToolMeshComponent@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolMeshComponent@@UEAA@XZ
?GetStaticMemoryPool@URedSplineToolMeshComponent@@SA?AUFMemoryPoolHandle@@XZ
?OnRegister@URedSplineToolMeshComponent@@UEAAXXZ
?OnUnregister@URedSplineToolMeshComponent@@UEAAXXZ
?_getUObject@URedSplineToolSplineMeshComponent@@UEBAPEAVUObject@@XZ
?Z_Construct_UClass_URedSplineToolRVTDecalComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolRVTDecalComponent@@SAPEAVUClass@@XZ
??0URedSplineToolRVTDecalComponent@@QEAA@AEBVFObjectInitializer@@@Z
??0URedSplineToolRVTDecalComponent@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolRVTDecalComponent@@UEAA@XZ
?Z_Construct_UClass_URedSplineToolStaticRVTDecalComponent@@YAPEAVUClass@@W4ETypeConstructPhase@@@Z
?StaticClass@URedSplineToolStaticRVTDecalComponent@@SAPEAVUClass@@XZ
??0URedSplineToolStaticRVTDecalComponent@@QEAA@AEBVFObjectInitializer@@@Z
??0URedSplineToolStaticRVTDecalComponent@@QEAA@AEAVFVTableHelper@@@Z
??1URedSplineToolStaticRVTDecalComponent@@UEAA@XZ
?GetComponent@ARedSplineTool@@UEAAPEAVURedSplineToolComponent@@XZ
?GetComponent@ARedSplineTool@@UEBAPEAVURedSplineToolComponent@@XZ
?GetShowFlagName@URedSplineToolComponent@@UEAA?AVFString@@XZ
?GetMainSpline@URedSplineToolComponent@@UEAAPEAVURedSplineToolSpline@@XZ
?GetMainSplineConst@URedSplineToolComponent@@UEBAPEAVURedSplineToolSpline@@XZ
?GetNumMaterials@URedSplineToolComponent@@UEBAHXZ
?GetUsedMaterials@URedSplineToolComponent@@UEBAXAEAV?$TArray@PEAVUMaterialInterface@@V?$TSizedDefaultAllocator@$0CA@@@@@_N@Z
?IsFreezable@URedSplineToolLandscapeComponent@@UEAA_NXZ
?IsFrozen@URedSplineToolLandscapeComponent@@UEAA_NXZ
?ToggleFrozen@URedSplineToolLandscapeComponent@@UEAAX_N@Z
?GetMemoryPool@URedSplineToolMeshComponent@@UEAA?AUFMemoryPoolHandle@@XZ
?IsFreezable@URedSplineToolMeshComponent@@UEAA_NXZ
?IsFrozen@URedSplineToolMeshComponent@@UEAA_NXZ
?ToggleFrozen@URedSplineToolMeshComponent@@UEAAX_N@Z
?IsFreezable@URedSplineToolSplineMeshComponent@@UEAA_NXZ
?IsFrozen@URedSplineToolSplineMeshComponent@@UEAA_NXZ
?ToggleFrozen@URedSplineToolSplineMeshComponent@@UEAAX_N@Z
?OnRegister@URedSplineToolStaticRVTDecalComponent@@UEAAXXZ
?OnComponentCreated@URedSplineToolStaticRVTDecalComponent@@UEAAXXZ
?SetDrawSortPriority@URedSplineToolStaticRVTDecalComponent@@UEAAXH@Z
?UpdateRenderPriority@URedSplineToolStaticRVTDecalComponent@@UEAAXXZ
?PostEditChangeProperty@URedSplineToolStaticRVTDecalComponent@@UEAAXAEAUFPropertyChangedEvent@@@Z
?ShouldMoveHandlesMirrored@URedSplineToolSpline@@UEBA_NXZ
?ShouldAlignToSurfaceNormalOnSnap@URedSplineToolSpline@@UEBA_NXZ
?ShouldAutoSmooth@URedSplineToolSpline@@UEBA_NXZ
?ShouldShowAllControlPointsHandles@URedSplineToolSpline@@UEBA_NXZ
?ShouldShowControlPointWidthValues@URedSplineToolSpline@@UEBA_NXZ
?ShouldShowRollVisualization@URedSplineToolSpline@@UEBA_NXZ
?OnUnregister@URedSplineToolSplineMeshComponent@@UEAAXXZ
?PostEditChangeProperty@URedSplineToolSplineMeshComponent@@UEAAXAEAUFPropertyChangedEvent@@@Z
?ClearSplineMeshComponents@URedSplineToolSplineMeshComponent@@MEAAXXZ
?GetAssetRegistryTags@URedSplineToolPreset@@UEBAXVFAssetRegistryTagsContext@@@Z
?OnRegister@URedSplineToolComponent@@UEAAXXZ
?OnPostRegisterAllOwnerComponents@URedSplineToolComponent@@UEAAXXZ
?OnComponentCreated@URedSplineToolComponent@@UEAAXXZ
?PostEditChangeChainProperty@URedSplineToolComponent@@UEAAXAEAUFPropertyChangedChainEvent@@@Z
?OnUnregister@URedSplineToolComponent@@UEAAXXZ
?CalcBounds@URedSplineToolComponent@@UEBA?AU?$TBoxSphereBounds@NN@Math@UE@@AEBU?$TTransform@N@34@@Z
?CreateSceneProxy@URedSplineToolComponent@@UEAAPEAVFPrimitiveSceneProxy@@XZ
```

</details>

<details><summary>KERNEL32.dll — 18 functions</summary>

```
InitializeSListHead
GetSystemTimeAsFileTime
GetCurrentProcessId
QueryPerformanceCounter
TerminateProcess
GetCurrentProcess
GetModuleHandleW
IsProcessorFeaturePresent
GetStartupInfoW
SetUnhandledExceptionFilter
UnhandledExceptionFilter
IsDebuggerPresent
RtlVirtualUnwind
RtlLookupFunctionEntry
RtlCaptureContext
GetCurrentThreadId
FlsGetValue
DisableThreadLibraryCalls
```

</details>

<details><summary>VCRUNTIME140.dll — 9 functions</summary>

```
__std_terminate
__C_specific_handler
__std_type_info_destroy_list
__current_exception_context
memcpy
memmove
memset
_purecall
__current_exception
```

</details>

<details><summary>VCRUNTIME140_1.dll — 1 functions</summary>

```
__CxxFrameHandler4
```

</details>

<details><summary>api-ms-win-crt-string-l1-1-0.dll — 2 functions</summary>

```
strlen
wcslen
```

</details>

<details><summary>api-ms-win-crt-math-l1-1-0.dll — 2 functions</summary>

```
cosf
logf
```

</details>

<details><summary>api-ms-win-crt-runtime-l1-1-0.dll — 12 functions</summary>

```
_register_onexit_function
terminate
_initterm_e
_initterm
_cexit
_crt_at_quick_exit
_crt_atexit
_execute_onexit_table
_configure_narrow_argv
_initialize_onexit_table
_initialize_narrow_environment
_seh_filter_dll
```

</details>

## Exports (407 symbols)

### Top contributors by owning type/symbol group

| Count | Type |
|---|---|
| 41 | URedRoadsSplineNew |
| 34 | URedRoadsSegmentComponent |
| 32 | URedRoadsJunctionComponent |
| 25 | &lt;other&gt; |
| 24 | URedRoadsMeshComponent |
| 24 | URedRoadsStaticRVTDecalComponent |
| 23 | URedRoadsElementComponent |
| 19 | URedRoadsLandscapeComponent |
| 19 | URedRoadsRVTDecalComponent |
| 18 | ARedRoadsJunction |
| 18 | ARedRoadsSegment |
| 17 | ARedRoadsElement |
| 17 | URedRoadsSettings |
| 15 | URedRoadsProjectSettings |
| 14 | URedRoadsPreset |
| 13 | URedRoadsSplineControlPoint |
| 11 | FRedRoadsModule |
| 9 | FJunctionSideSplinePointData |
| 8 | FRedRoadsSpline |
| 8 | RedRoadsUtils |
| 5 | FJunctionToSegmentConnection |
| 4 | FRedRoadsControlPoint |
| 4 | FRedRoadsEnd |
| 4 | FRedRoadsTier |
| 1 | UE |

<details><summary>All export symbols</summary>

| Ordinal | RVA | Name |
|---|---|---|
| 1 | 0x00009F30 | `class UEnum * __ptr64 __cdecl StaticEnum&lt;enum ERedRoadTier&gt;(void)` |
| 2 | 0x00009F70 | `class UEnum * __ptr64 __cdecl StaticEnum&lt;enum ERedRoadsEndType&gt;(void)` |
| 3 | 0x0000A000 | `public: __cdecl ARedRoadsElement::ARedRoadsElement(class FVTableHelper & __ptr64) __ptr64` |
| 4 | 0x0000A030 | `public: __cdecl ARedRoadsElement::ARedRoadsElement(class FObjectInitializer const & __ptr64) __ptr64` |
| 5 | 0x0000A060 | `public: __cdecl ARedRoadsJunction::ARedRoadsJunction(class FVTableHelper & __ptr64) __ptr64` |
| 6 | 0x0000A090 | `public: __cdecl ARedRoadsJunction::ARedRoadsJunction(class FObjectInitializer const & __ptr64) __ptr64` |
| 7 | 0x0000A160 | `public: __cdecl ARedRoadsSegment::ARedRoadsSegment(class FVTableHelper & __ptr64) __ptr64` |
| 8 | 0x0000A190 | `public: __cdecl ARedRoadsSegment::ARedRoadsSegment(class FObjectInitializer const & __ptr64) __ptr64` |
| 9 | 0x0001A680 | `public: __cdecl FJunctionSideSplinePointData::FJunctionSideSplinePointData(int,class URedUniversalSplineControlPoint * __ptr64,class URedUniversalSplineControlPoint * __ptr64,struct UE::Math::TVector&lt;double&gt;) __ptr64` |
| 10 | 0x0001A6B0 | `public: __cdecl FJunctionSideSplinePointData::FJunctionSideSplinePointData(class ARedRoadsJunction * __ptr64,int,struct UE::Math::TVector&lt;double&gt; const & __ptr64) __ptr64` |
| 11 | 0x0000A260 | `public: __cdecl FJunctionSideSplinePointData::FJunctionSideSplinePointData(void) __ptr64` |
| 12 | 0x0000A2A0 | `public: __cdecl FRedRoadsControlPoint::FRedRoadsControlPoint(void) __ptr64` |
| 13 | 0x0000A3C0 | `public: __cdecl FRedRoadsEnd::FRedRoadsEnd(void) __ptr64` |
| 14 | 0x0000A3F0 | `public: __cdecl FRedRoadsModule::FRedRoadsModule(class FRedRoadsModule && __ptr64) __ptr64` |
| 15 | 0x0000A420 | `public: __cdecl FRedRoadsModule::FRedRoadsModule(class FRedRoadsModule const & __ptr64) __ptr64` |
| 16 | 0x0000A450 | `public: __cdecl FRedRoadsModule::FRedRoadsModule(void) __ptr64` |
| 17 | 0x0000A470 | `public: __cdecl FRedRoadsSpline::FRedRoadsSpline(struct FRedRoadsSpline && __ptr64) __ptr64` |
| 18 | 0x0000A4B0 | `public: __cdecl FRedRoadsSpline::FRedRoadsSpline(struct FRedRoadsSpline const & __ptr64) __ptr64` |
| 19 | 0x0000A540 | `public: __cdecl FRedRoadsSpline::FRedRoadsSpline(float) __ptr64` |
| 20 | 0x0000A560 | `public: __cdecl FRedRoadsSpline::FRedRoadsSpline(void) __ptr64` |
| 21 | 0x0000A580 | `public: __cdecl FRedRoadsTier::FRedRoadsTier(void) __ptr64` |
| 22 | 0x0000A700 | `public: __cdecl URedRoadsElementComponent::URedRoadsElementComponent(class FVTableHelper & __ptr64) __ptr64` |
| 23 | 0x0000A7E0 | `public: __cdecl URedRoadsElementComponent::URedRoadsElementComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 24 | 0x0000A8C0 | `public: __cdecl URedRoadsJunctionComponent::URedRoadsJunctionComponent(class FVTableHelper & __ptr64) __ptr64` |
| 25 | 0x000220D0 | `public: __cdecl URedRoadsJunctionComponent::URedRoadsJunctionComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 26 | 0x0000A970 | `public: __cdecl URedRoadsLandscapeComponent::URedRoadsLandscapeComponent(class FVTableHelper & __ptr64) __ptr64` |
| 27 | 0x0000A9C0 | `public: __cdecl URedRoadsLandscapeComponent::URedRoadsLandscapeComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 28 | 0x0000AA10 | `public: __cdecl URedRoadsMeshComponent::URedRoadsMeshComponent(class FVTableHelper & __ptr64) __ptr64` |
| 29 | 0x0000AAB0 | `public: __cdecl URedRoadsMeshComponent::URedRoadsMeshComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 30 | 0x0000AB50 | `public: __cdecl URedRoadsPreset::URedRoadsPreset(class FVTableHelper & __ptr64) __ptr64` |
| 31 | 0x0000AB80 | `public: __cdecl URedRoadsPreset::URedRoadsPreset(class FObjectInitializer const & __ptr64) __ptr64` |
| 32 | 0x0000ABB0 | `public: __cdecl URedRoadsProjectSettings::URedRoadsProjectSettings(class FVTableHelper & __ptr64) __ptr64` |
| 33 | 0x0000ABF0 | `public: __cdecl URedRoadsProjectSettings::URedRoadsProjectSettings(class FObjectInitializer const & __ptr64) __ptr64` |
| 34 | 0x0000AC30 | `public: __cdecl URedRoadsRVTDecalComponent::URedRoadsRVTDecalComponent(class FVTableHelper & __ptr64) __ptr64` |
| 35 | 0x0000AC80 | `public: __cdecl URedRoadsRVTDecalComponent::URedRoadsRVTDecalComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 36 | 0x0000ACD0 | `public: __cdecl URedRoadsSegmentComponent::URedRoadsSegmentComponent(class FVTableHelper & __ptr64) __ptr64` |
| 37 | 0x00025420 | `public: __cdecl URedRoadsSegmentComponent::URedRoadsSegmentComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 38 | 0x0000ADE0 | `public: __cdecl URedRoadsSettings::URedRoadsSettings(class FVTableHelper & __ptr64) __ptr64` |
| 39 | 0x0000AF50 | `public: __cdecl URedRoadsSettings::URedRoadsSettings(class FObjectInitializer const & __ptr64) __ptr64` |
| 40 | 0x0000B0C0 | `public: __cdecl URedRoadsSplineControlPoint::URedRoadsSplineControlPoint(class FVTableHelper & __ptr64) __ptr64` |
| 41 | 0x0000B0F0 | `public: __cdecl URedRoadsSplineControlPoint::URedRoadsSplineControlPoint(void) __ptr64` |
| 42 | 0x0000B120 | `public: __cdecl URedRoadsSplineNew::URedRoadsSplineNew(class FVTableHelper & __ptr64) __ptr64` |
| 43 | 0x0000B150 | `public: __cdecl URedRoadsSplineNew::URedRoadsSplineNew(void) __ptr64` |
| 44 | 0x0000B180 | `public: __cdecl URedRoadsStaticRVTDecalComponent::URedRoadsStaticRVTDecalComponent(class FVTableHelper & __ptr64) __ptr64` |
| 45 | 0x0000B220 | `public: __cdecl URedRoadsStaticRVTDecalComponent::URedRoadsStaticRVTDecalComponent(class FObjectInitializer const & __ptr64) __ptr64` |
| 46 | 0x0000B3A0 | `public: virtual __cdecl ARedRoadsElement::~ARedRoadsElement(void) __ptr64` |
| 47 | 0x0000B3C0 | `public: virtual __cdecl ARedRoadsJunction::~ARedRoadsJunction(void) __ptr64` |
| 48 | 0x0000B3E0 | `public: virtual __cdecl ARedRoadsSegment::~ARedRoadsSegment(void) __ptr64` |
| 49 | 0x0000B440 | `public: virtual __cdecl FRedRoadsModule::~FRedRoadsModule(void) __ptr64` |
| 50 | 0x0000B450 | `public: __cdecl FRedRoadsSpline::~FRedRoadsSpline(void) __ptr64` |
| 51 | 0x0000B500 | `public: virtual __cdecl URedRoadsElementComponent::~URedRoadsElementComponent(void) __ptr64` |
| 52 | 0x0000B580 | `public: virtual __cdecl URedRoadsJunctionComponent::~URedRoadsJunctionComponent(void) __ptr64` |
| 53 | 0x0000B640 | `public: virtual __cdecl URedRoadsLandscapeComponent::~URedRoadsLandscapeComponent(void) __ptr64` |
| 54 | 0x0000B680 | `public: virtual __cdecl URedRoadsMeshComponent::~URedRoadsMeshComponent(void) __ptr64` |
| 55 | 0x0000B710 | `public: virtual __cdecl URedRoadsPreset::~URedRoadsPreset(void) __ptr64` |
| 56 | 0x0000B730 | `public: virtual __cdecl URedRoadsProjectSettings::~URedRoadsProjectSettings(void) __ptr64` |
| 57 | 0x0000B770 | `public: virtual __cdecl URedRoadsRVTDecalComponent::~URedRoadsRVTDecalComponent(void) __ptr64` |
| 58 | 0x0000B7B0 | `public: virtual __cdecl URedRoadsSegmentComponent::~URedRoadsSegmentComponent(void) __ptr64` |
| 59 | 0x0000B850 | `public: virtual __cdecl URedRoadsSettings::~URedRoadsSettings(void) __ptr64` |
| 60 | 0x0000B8E0 | `public: virtual __cdecl URedRoadsSplineControlPoint::~URedRoadsSplineControlPoint(void) __ptr64` |
| 61 | 0x0000B900 | `public: virtual __cdecl URedRoadsSplineNew::~URedRoadsSplineNew(void) __ptr64` |
| 62 | 0x0000B920 | `public: virtual __cdecl URedRoadsStaticRVTDecalComponent::~URedRoadsStaticRVTDecalComponent(void) __ptr64` |
| 63 | 0x0000BE60 | `public: static void * __ptr64 __cdecl ARedRoadsElement::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 64 | 0x0000BE70 | `public: static void * __ptr64 __cdecl ARedRoadsElement::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 65 | 0x0000BEF0 | `public: static void * __ptr64 __cdecl ARedRoadsJunction::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 66 | 0x0000BF00 | `public: static void * __ptr64 __cdecl ARedRoadsJunction::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 67 | 0x0000BF80 | `public: static void * __ptr64 __cdecl ARedRoadsSegment::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 68 | 0x0000BF90 | `public: static void * __ptr64 __cdecl ARedRoadsSegment::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 69 | 0x0000C010 | `public: static void * __ptr64 __cdecl URedRoadsElementComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 70 | 0x0000C020 | `public: static void * __ptr64 __cdecl URedRoadsElementComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 71 | 0x0000C0A0 | `public: static void * __ptr64 __cdecl URedRoadsJunctionComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 72 | 0x0000C0B0 | `public: static void * __ptr64 __cdecl URedRoadsJunctionComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 73 | 0x0000C130 | `public: static void * __ptr64 __cdecl URedRoadsLandscapeComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 74 | 0x0000C140 | `public: static void * __ptr64 __cdecl URedRoadsLandscapeComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 75 | 0x0000C1C0 | `public: static void * __ptr64 __cdecl URedRoadsMeshComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 76 | 0x0000C1D0 | `public: static void * __ptr64 __cdecl URedRoadsMeshComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 77 | 0x0000C250 | `public: static void * __ptr64 __cdecl URedRoadsPreset::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 78 | 0x0000C260 | `public: static void * __ptr64 __cdecl URedRoadsPreset::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 79 | 0x0000C2E0 | `public: static void * __ptr64 __cdecl URedRoadsProjectSettings::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 80 | 0x0000C2F0 | `public: static void * __ptr64 __cdecl URedRoadsProjectSettings::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 81 | 0x0000C370 | `public: static void * __ptr64 __cdecl URedRoadsRVTDecalComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 82 | 0x0000C380 | `public: static void * __ptr64 __cdecl URedRoadsRVTDecalComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 83 | 0x0000C400 | `public: static void * __ptr64 __cdecl URedRoadsSegmentComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 84 | 0x0000C410 | `public: static void * __ptr64 __cdecl URedRoadsSegmentComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 85 | 0x0000C490 | `public: static void * __ptr64 __cdecl URedRoadsSettings::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 86 | 0x0000C4A0 | `public: static void * __ptr64 __cdecl URedRoadsSettings::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 87 | 0x0000C520 | `public: static void * __ptr64 __cdecl URedRoadsSplineControlPoint::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 88 | 0x0000C530 | `public: static void * __ptr64 __cdecl URedRoadsSplineControlPoint::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 89 | 0x0000C5B0 | `public: static void * __ptr64 __cdecl URedRoadsSplineNew::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 90 | 0x0000C5C0 | `public: static void * __ptr64 __cdecl URedRoadsSplineNew::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 91 | 0x0000C640 | `public: static void * __ptr64 __cdecl URedRoadsStaticRVTDecalComponent::operator new(unsigned __int64,enum EInternal * __ptr64)` |
| 92 | 0x0000C650 | `public: static void * __ptr64 __cdecl URedRoadsStaticRVTDecalComponent::operator new(unsigned __int64,enum EInternal,class UObject * __ptr64,class FName,enum EObjectFlags)` |
| 93 | 0x0000CC50 | `public: static void __cdecl ARedRoadsElement::operator delete(void * __ptr64)` |
| 94 | 0x0000CC60 | `public: static void __cdecl ARedRoadsJunction::operator delete(void * __ptr64)` |
| 95 | 0x0000CC70 | `public: static void __cdecl ARedRoadsSegment::operator delete(void * __ptr64)` |
| 96 | 0x0000CC80 | `public: static void __cdecl URedRoadsElementComponent::operator delete(void * __ptr64)` |
| 97 | 0x0000CC90 | `public: static void __cdecl URedRoadsJunctionComponent::operator delete(void * __ptr64)` |
| 98 | 0x0000CCA0 | `public: static void __cdecl URedRoadsLandscapeComponent::operator delete(void * __ptr64)` |
| 99 | 0x0000CCB0 | `public: static void __cdecl URedRoadsMeshComponent::operator delete(void * __ptr64)` |
| 100 | 0x0000CCC0 | `public: static void __cdecl URedRoadsPreset::operator delete(void * __ptr64)` |
| 101 | 0x0000CCD0 | `public: static void __cdecl URedRoadsProjectSettings::operator delete(void * __ptr64)` |
| 102 | 0x0000CCE0 | `public: static void __cdecl URedRoadsRVTDecalComponent::operator delete(void * __ptr64)` |
| 103 | 0x0000CCF0 | `public: static void __cdecl URedRoadsSegmentComponent::operator delete(void * __ptr64)` |
| 104 | 0x0000CD00 | `public: static void __cdecl URedRoadsSettings::operator delete(void * __ptr64)` |
| 105 | 0x0000CD10 | `public: static void __cdecl URedRoadsSplineControlPoint::operator delete(void * __ptr64)` |
| 106 | 0x0000CD20 | `public: static void __cdecl URedRoadsSplineNew::operator delete(void * __ptr64)` |
| 107 | 0x0000CD30 | `public: static void __cdecl URedRoadsStaticRVTDecalComponent::operator delete(void * __ptr64)` |
| 108 | 0x0000CD40 | `public: struct FJunctionSideSplinePointData & __ptr64 __cdecl FJunctionSideSplinePointData::operator=(struct FJunctionSideSplinePointData && __ptr64) __ptr64` |
| 109 | 0x0000CD70 | `public: struct FJunctionSideSplinePointData & __ptr64 __cdecl FJunctionSideSplinePointData::operator=(struct FJunctionSideSplinePointData const & __ptr64) __ptr64` |
| 110 | 0x0000CD90 | `public: struct FRedRoadsControlPoint & __ptr64 __cdecl FRedRoadsControlPoint::operator=(struct FRedRoadsControlPoint && __ptr64) __ptr64` |
| 111 | 0x0000CE90 | `public: struct FRedRoadsControlPoint & __ptr64 __cdecl FRedRoadsControlPoint::operator=(struct FRedRoadsControlPoint const & __ptr64) __ptr64` |
| 112 | 0x0000CF20 | `public: struct FRedRoadsEnd & __ptr64 __cdecl FRedRoadsEnd::operator=(struct FRedRoadsEnd && __ptr64) __ptr64` |
| 113 | 0x0000CF50 | `public: struct FRedRoadsEnd & __ptr64 __cdecl FRedRoadsEnd::operator=(struct FRedRoadsEnd const & __ptr64) __ptr64` |
| 114 | 0x0000CF80 | `public: class FRedRoadsModule & __ptr64 __cdecl FRedRoadsModule::operator=(class FRedRoadsModule && __ptr64) __ptr64` |
| 115 | 0x0000CFA0 | `public: class FRedRoadsModule & __ptr64 __cdecl FRedRoadsModule::operator=(class FRedRoadsModule const & __ptr64) __ptr64` |
| 116 | 0x0000CFC0 | `public: struct FRedRoadsSpline & __ptr64 __cdecl FRedRoadsSpline::operator=(struct FRedRoadsSpline && __ptr64) __ptr64` |
| 117 | 0x0000D020 | `public: struct FRedRoadsSpline & __ptr64 __cdecl FRedRoadsSpline::operator=(struct FRedRoadsSpline const & __ptr64) __ptr64` |
| 118 | 0x0000D0C0 | `public: struct FRedRoadsTier & __ptr64 __cdecl FRedRoadsTier::operator=(struct FRedRoadsTier && __ptr64) __ptr64` |
| 119 | 0x0000D100 | `public: struct FRedRoadsTier & __ptr64 __cdecl FRedRoadsTier::operator=(struct FRedRoadsTier const & __ptr64) __ptr64` |
| 120 | 0x0003B0A0 | `const ARedRoadsElement::`vftable'` |
| 121 | 0x0003CAF8 | `const ARedRoadsJunction::`vftable'` |
| 122 | 0x00041AC0 | `const ARedRoadsSegment::`vftable'` |
| 123 | 0x00044628 | `const FRedRoadsModule::`vftable'` |
| 124 | 0x0003C8A8 | `const URedRoadsElementComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 125 | 0x0003C868 | `const URedRoadsElementComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 126 | 0x0003C990 | `const URedRoadsElementComponent::`vftable'{for `IInterface_AsyncCompilation'}` |
| 127 | 0x0003C910 | `const URedRoadsElementComponent::`vftable'{for `INavRelevantInterface'}` |
| 128 | 0x0003C9D8 | `const URedRoadsElementComponent::`vftable'{for `IPhysicsBodyInstanceOwner'}` |
| 129 | 0x0003CAA8 | `const URedRoadsElementComponent::`vftable'{for `IPhysicsBodyInstanceOwnerResolver'}` |
| 130 | 0x0003C9A8 | `const URedRoadsElementComponent::`vftable'{for `IPhysicsComponent'}` |
| 131 | 0x0003BB90 | `const URedRoadsElementComponent::`vftable'{for `UObject'}` |
| 132 | 0x0003EE00 | `const URedRoadsJunctionComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 133 | 0x0003EDC0 | `const URedRoadsJunctionComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 134 | 0x0003EEE8 | `const URedRoadsJunctionComponent::`vftable'{for `IInterface_AsyncCompilation'}` |
| 135 | 0x0003EE68 | `const URedRoadsJunctionComponent::`vftable'{for `INavRelevantInterface'}` |
| 136 | 0x0003EF30 | `const URedRoadsJunctionComponent::`vftable'{for `IPhysicsBodyInstanceOwner'}` |
| 137 | 0x0003F000 | `const URedRoadsJunctionComponent::`vftable'{for `IPhysicsBodyInstanceOwnerResolver'}` |
| 138 | 0x0003EF00 | `const URedRoadsJunctionComponent::`vftable'{for `IPhysicsComponent'}` |
| 139 | 0x0003E0E8 | `const URedRoadsJunctionComponent::`vftable'{for `UObject'}` |
| 140 | 0x00040098 | `const URedRoadsLandscapeComponent::`vftable'` |
| 141 | 0x00040030 | `const URedRoadsLandscapeComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 142 | 0x0003FFF0 | `const URedRoadsLandscapeComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 143 | 0x0003F990 | `const URedRoadsLandscapeComponent::`vftable'{for `UObject'}` |
| 144 | 0x00041120 | `const URedRoadsMeshComponent::`vftable'` |
| 145 | 0x00040F08 | `const URedRoadsMeshComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 146 | 0x00040EC8 | `const URedRoadsMeshComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 147 | 0x00040FF0 | `const URedRoadsMeshComponent::`vftable'{for `IInterface_AsyncCompilation'}` |
| 148 | 0x00040F70 | `const URedRoadsMeshComponent::`vftable'{for `INavRelevantInterface'}` |
| 149 | 0x00041038 | `const URedRoadsMeshComponent::`vftable'{for `IPhysicsBodyInstanceOwner'}` |
| 150 | 0x00041108 | `const URedRoadsMeshComponent::`vftable'{for `IPhysicsBodyInstanceOwnerResolver'}` |
| 151 | 0x00041008 | `const URedRoadsMeshComponent::`vftable'{for `IPhysicsComponent'}` |
| 152 | 0x00040110 | `const URedRoadsMeshComponent::`vftable'{for `UObject'}` |
| 153 | 0x0003D570 | `const URedRoadsPreset::`vftable'` |
| 154 | 0x0003DD08 | `const URedRoadsProjectSettings::`vftable'` |
| 155 | 0x00041A48 | `const URedRoadsRVTDecalComponent::`vftable'` |
| 156 | 0x000419E0 | `const URedRoadsRVTDecalComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 157 | 0x000419A0 | `const URedRoadsRVTDecalComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 158 | 0x000411B0 | `const URedRoadsRVTDecalComponent::`vftable'{for `UObject'}` |
| 159 | 0x00043248 | `const URedRoadsSegmentComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 160 | 0x00043208 | `const URedRoadsSegmentComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 161 | 0x00043330 | `const URedRoadsSegmentComponent::`vftable'{for `IInterface_AsyncCompilation'}` |
| 162 | 0x000432B0 | `const URedRoadsSegmentComponent::`vftable'{for `INavRelevantInterface'}` |
| 163 | 0x00043378 | `const URedRoadsSegmentComponent::`vftable'{for `IPhysicsBodyInstanceOwner'}` |
| 164 | 0x00043448 | `const URedRoadsSegmentComponent::`vftable'{for `IPhysicsBodyInstanceOwnerResolver'}` |
| 165 | 0x00043348 | `const URedRoadsSegmentComponent::`vftable'{for `IPhysicsComponent'}` |
| 166 | 0x00042530 | `const URedRoadsSegmentComponent::`vftable'{for `UObject'}` |
| 167 | 0x0003D920 | `const URedRoadsSettings::`vftable'` |
| 168 | 0x0003F018 | `const URedRoadsSplineControlPoint::`vftable'` |
| 169 | 0x0003F3D8 | `const URedRoadsSplineNew::`vftable'` |
| 170 | 0x00044588 | `const URedRoadsStaticRVTDecalComponent::`vftable'` |
| 171 | 0x00044370 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `IAsyncPhysicsStateProcessor'}` |
| 172 | 0x00044330 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `IInterface_AssetUserData'}` |
| 173 | 0x00044458 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `IInterface_AsyncCompilation'}` |
| 174 | 0x000443D8 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `INavRelevantInterface'}` |
| 175 | 0x000444A0 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `IPhysicsBodyInstanceOwner'}` |
| 176 | 0x00044570 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `IPhysicsBodyInstanceOwnerResolver'}` |
| 177 | 0x00044470 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `IPhysicsComponent'}` |
| 178 | 0x00043568 | `const URedRoadsStaticRVTDecalComponent::`vftable'{for `UObject'}` |
| 179 | 0x0000E3F0 | `public: void __cdecl ARedRoadsJunction::`default constructor closure'(void) __ptr64` |
| 180 | 0x0000E420 | `public: void __cdecl ARedRoadsSegment::`default constructor closure'(void) __ptr64` |
| 181 | 0x0000E450 | `public: void __cdecl URedRoadsLandscapeComponent::`default constructor closure'(void) __ptr64` |
| 182 | 0x0000E4B0 | `public: void __cdecl URedRoadsMeshComponent::`default constructor closure'(void) __ptr64` |
| 183 | 0x0000E4E0 | `public: void __cdecl URedRoadsPreset::`default constructor closure'(void) __ptr64` |
| 184 | 0x0000E520 | `public: void __cdecl URedRoadsProjectSettings::`default constructor closure'(void) __ptr64` |
| 185 | 0x0000E560 | `public: void __cdecl URedRoadsRVTDecalComponent::`default constructor closure'(void) __ptr64` |
| 186 | 0x0000E5C0 | `public: void __cdecl URedRoadsSettings::`default constructor closure'(void) __ptr64` |
| 187 | 0x0000E5F0 | `public: void __cdecl URedRoadsStaticRVTDecalComponent::`default constructor closure'(void) __ptr64` |
| 188 | 0x00014E30 | `public: virtual bool __cdecl URedRoadsSplineNew::CanSetSplineWidth(void)const __ptr64` |
| 189 | 0x00014E50 | `public: virtual class URedUniversalSplineControlPoint * __ptr64 __cdecl URedRoadsSplineNew::CreateControlPoint(void) __ptr64` |
| 190 | 0x00022750 | `public: virtual void __cdecl URedRoadsJunctionComponent::CreateMainSpline(void) __ptr64` |
| 191 | 0x000258E0 | `public: virtual void __cdecl URedRoadsSegmentComponent::CreateMainSpline(void) __ptr64` |
| 192 | 0x0001C920 | `bool __cdecl RedRoadsUtils::CreateSmoothRoadsSegmentToRoadsJunctionConnection(struct FJunctionSideSplinePointData & __ptr64,class URedUniversalSplineControlPoint * __ptr64)` |
| 193 | 0x00025E30 | `public: void __cdecl URedRoadsSegmentComponent::DrawEndConnectionIndicator(class FPrimitiveDrawInterface * __ptr64,struct FJunctionToSegmentConnection const & __ptr64)const __ptr64` |
| 194 | 0x00022DA0 | `public: void __cdecl URedRoadsJunctionComponent::DrawSideConnectionIndicator(class FPrimitiveDrawInterface & __ptr64,struct FJunctionToSegmentConnection const & __ptr64)const __ptr64` |
| 195 | 0x000230F0 | `public: virtual void __cdecl URedRoadsJunctionComponent::DrawVisualization(class FMeshElementCollector * __ptr64,class FSceneView const * __ptr64,int,bool,bool,class HHitProxy * __ptr64)const __ptr64` |
| 196 | 0x00026180 | `public: virtual void __cdecl URedRoadsSegmentComponent::DrawVisualization(class FMeshElementCollector * __ptr64,class FSceneView const * __ptr64,int,bool,bool,class HHitProxy * __ptr64)const __ptr64` |
| 197 | 0x0001CE80 | `int __cdecl RedRoadsUtils::FindAllSegmentConnectionsInJunction(class URedRoadsJunctionComponent * __ptr64,class TArray&lt;struct FJunctionToSegmentConnection,class TSizedDefaultAllocator&lt;32&gt; &gt; & __ptr64)` |
| 198 | 0x00010340 | `private: void __cdecl FRedRoadsModule::ForceClassDefaultObjectsCreation(void) __ptr64` |
| 199 | 0x00014F40 | `public: virtual float __cdecl URedRoadsSplineNew::GetAutoSmoothingIntensity(void)const __ptr64` |
| 200 | 0x00010420 | `public: virtual class FName __cdecl URedRoadsSettings::GetCategoryName(void)const __ptr64` |
| 201 | 0x0001D2A0 | `bool __cdecl RedRoadsUtils::GetClosestSideInAllRoadsJunctions(struct UE::Math::TVector&lt;double&gt;,struct FJunctionSideSplinePointData & __ptr64,bool)` |
| 202 | 0x0001D500 | `bool __cdecl RedRoadsUtils::GetClosestSideInRoadsJunction(class URedRoadsJunctionComponent * __ptr64,struct UE::Math::TVector&lt;double&gt; const & __ptr64,int,struct FJunctionSideSplinePointData & __ptr64)` |
| 203 | 0x0001D630 | `int __cdecl RedRoadsUtils::GetClosestStraightSidesInRoadsJunction(class URedRoadsJunctionComponent * __ptr64,struct UE::Math::TVector&lt;double&gt; const & __ptr64,int,class TArray&lt;struct FJunctionSideSplinePointData,class TSizedDefaultAllocator&lt;32&gt; &gt; & __ptr64)` |
| 204 | 0x000104C0 | `public: class TArray&lt;struct FJunctionToSegmentConnection,class TSizedDefaultAllocator&lt;32&gt; &gt; & __ptr64 __cdecl URedRoadsJunctionComponent::GetConnections(void) __ptr64` |
| 205 | 0x00014F60 | `public: virtual struct FLinearColor __cdecl URedRoadsSplineNew::GetControlPointGizmoColor(void)const __ptr64` |
| 206 | 0x00014FC0 | `public: virtual float __cdecl URedRoadsSplineNew::GetControlPointGizmoSize(void)const __ptr64` |
| 207 | 0x00014FE0 | `public: virtual float __cdecl URedRoadsSplineNew::GetControlPointHandleGizmoSize(void)const __ptr64` |
| 208 | 0x00026230 | `public: class TArray&lt;class URedUniversalSplineControlPoint * __ptr64,class TSizedDefaultAllocator&lt;32&gt; &gt; __cdecl URedRoadsSegmentComponent::GetControlPointsOutsideOfTierLimits(void) __ptr64` |
| 209 | 0x00010570 | `public: virtual struct TSoftObjectPtr&lt;class URedSplineToolPreset&gt; __cdecl ARedRoadsJunction::GetDefaultPreset(void) __ptr64` |
| 210 | 0x000105D0 | `public: virtual struct TSoftObjectPtr&lt;class URedSplineToolPreset&gt; __cdecl ARedRoadsSegment::GetDefaultPreset(void) __ptr64` |
| 211 | 0x0001DBA0 | `public: float __cdecl FJunctionSideSplinePointData::GetDistanceOnJunctionSideSplineSegment(void) __ptr64` |
| 212 | 0x00026400 | `public: struct FJunctionToSegmentConnection * __ptr64 __cdecl URedRoadsSegmentComponent::GetEndConnection(int) __ptr64` |
| 213 | 0x00015000 | `public: virtual float __cdecl URedRoadsSplineNew::GetFalloffSplineGizmoThickness(void)const __ptr64` |
| 214 | 0x00010740 | `public: virtual struct FRedRoadsTier const * __ptr64 __cdecl ARedRoadsElement::GetHeighestRedRoadTierFromComponents(void) __ptr64` |
| 215 | 0x000108E0 | `public: virtual enum ERedRoadTier __cdecl ARedRoadsElement::GetHeighestRoadTierFromComponents(void) __ptr64` |
| 216 | 0x00010B50 | `public: virtual class FString __cdecl ARedRoadsJunction::GetNewPresetSaveFolder(void)const __ptr64` |
| 217 | 0x00010B80 | `public: virtual class FString __cdecl ARedRoadsSegment::GetNewPresetSaveFolder(void)const __ptr64` |
| 218 | 0x00010CD0 | `public: virtual class UClass * __ptr64 __cdecl ARedRoadsJunction::GetPresetClass(void)const __ptr64` |
| 219 | 0x00010CE0 | `public: virtual class UClass * __ptr64 __cdecl ARedRoadsSegment::GetPresetClass(void)const __ptr64` |
| 220 | 0x00010D00 | `public: virtual struct FRedRoadsTier const * __ptr64 __cdecl URedRoadsElementComponent::GetRedRoadTier(void) __ptr64` |
| 221 | 0x00010DB0 | `public: virtual class TArray&lt;struct FRedRoadsTier const * __ptr64,class TSizedDefaultAllocator&lt;32&gt; &gt; __cdecl ARedRoadsElement::GetRedRoadTiersFromComponents(void)const __ptr64` |
| 222 | 0x0001DC30 | `public: struct UE::Math::TVector&lt;double&gt; __cdecl FJunctionSideSplinePointData::GetRightVectorOnJunctionSideSplineSegment(void) __ptr64` |
| 223 | 0x00010F80 | `public: virtual enum ERedRoadTier __cdecl URedRoadsElementComponent::GetRoadComponentTier(void) __ptr64` |
| 224 | 0x00010FB0 | `public: virtual class TArray&lt;enum ERedRoadTier,class TSizedDefaultAllocator&lt;32&gt; &gt; __cdecl ARedRoadsElement::GetRoadTiersFromComponents(void) __ptr64` |
| 225 | 0x0001DCF0 | `public: class ARedRoadsJunction * __ptr64 __cdecl FJunctionSideSplinePointData::GetRoadsJunction(void) __ptr64` |
| 226 | 0x0001DD70 | `public: class ARedRoadsJunction * __ptr64 __cdecl FJunctionToSegmentConnection::GetRoadsJunction(void) __ptr64` |
| 227 | 0x0001DE00 | `public: class URedRoadsJunctionComponent * __ptr64 __cdecl FJunctionSideSplinePointData::GetRoadsJunctionComponent(void) __ptr64` |
| 228 | 0x0001DE40 | `public: class URedRoadsJunctionComponent * __ptr64 __cdecl FJunctionToSegmentConnection::GetRoadsJunctionComponent(void) __ptr64` |
| 229 | 0x0001DE90 | `public: class URedRoadsJunctionComponent * __ptr64 __cdecl FJunctionToSegmentConnection::GetRoadsJunctionComponent(void)const __ptr64` |
| 230 | 0x0001DEE0 | `public: class URedRoadsSegmentComponent * __ptr64 __cdecl FJunctionSideSplinePointData::GetRoadsSegmentComponent(void) __ptr64` |
| 231 | 0x0001DF20 | `public: class URedRoadsSegmentComponent * __ptr64 __cdecl FJunctionToSegmentConnection::GetRoadsSegmentComponent(void) __ptr64` |
| 232 | 0x0001DF80 | `public: class URedRoadsSegmentComponent * __ptr64 __cdecl FJunctionToSegmentConnection::GetRoadsSegmentComponent(void)const __ptr64` |
| 233 | 0x0001DFE0 | `int __cdecl RedRoadsUtils::GetRoadsSegmentControlPointsInRadius(class URedSplineToolComponent const * __ptr64,struct UE::Math::TVector&lt;double&gt; const & __ptr64,int,class TArray&lt;class URedUniversalSplineControlPoint * __ptr64,class TSizedDefaultAllocator&lt;32&gt; &gt; & __ptr64,bool)` |
| 234 | 0x00011230 | `public: virtual class FName __cdecl URedRoadsSettings::GetSectionName(void)const __ptr64` |
| 235 | 0x00011290 | `public: virtual class FString __cdecl URedRoadsJunctionComponent::GetShowFlagName(void) __ptr64` |
| 236 | 0x00011320 | `public: virtual class FString __cdecl URedRoadsSegmentComponent::GetShowFlagName(void) __ptr64` |
| 237 | 0x00015020 | `public: virtual float __cdecl URedRoadsSplineNew::GetSideSplineGizmoThickness(void)const __ptr64` |
| 238 | 0x00015040 | `public: virtual struct FLinearColor __cdecl URedRoadsSplineNew::GetSplineGizmoColor(void)const __ptr64` |
| 239 | 0x000151A0 | `public: virtual float __cdecl URedRoadsSplineNew::GetSplineGizmoThickness(void)const __ptr64` |
| 240 | 0x000151D0 | `public: void __cdecl URedRoadsSplineNew::InitializeWithControlPoints(class TArray&lt;class URedRoadsSplineControlPoint * __ptr64,class TSizedDefaultAllocator&lt;32&gt; &gt;) __ptr64` |
| 241 | 0x000119C0 | `private: bool __cdecl URedRoadsRVTDecalComponent::IsAttachedToRoadsJunction(void) __ptr64` |
| 242 | 0x00011A00 | `private: bool __cdecl URedRoadsStaticRVTDecalComponent::IsAttachedToRoadsJunction(void) __ptr64` |
| 243 | 0x00011A40 | `private: bool __cdecl URedRoadsRVTDecalComponent::IsAttachedToRoadsSegment(void) __ptr64` |
| 244 | 0x00011A80 | `private: bool __cdecl URedRoadsStaticRVTDecalComponent::IsAttachedToRoadsSegment(void) __ptr64` |
| 245 | 0x0001E910 | `bool __cdecl RedRoadsUtils::IsSegmentControlPointConnectedToJunction(class URedUniversalSplineControlPoint * __ptr64,struct FJunctionSideSplinePointData & __ptr64,bool & __ptr64)` |
| 246 | 0x00011DF0 | `public: virtual void __cdecl URedRoadsJunctionComponent::OnPostRegisterAllOwnerComponents(void) __ptr64` |
| 247 | 0x00011E00 | `public: virtual void __cdecl URedRoadsSegmentComponent::OnPostRegisterAllOwnerComponents(void) __ptr64` |
| 248 | 0x000231D0 | `public: virtual void __cdecl URedRoadsJunctionComponent::OnRegister(void) __ptr64` |
| 249 | 0x00011E30 | `private: virtual void __cdecl URedRoadsLandscapeComponent::OnRegister(void) __ptr64` |
| 250 | 0x00011FE0 | `private: virtual void __cdecl URedRoadsMeshComponent::OnRegister(void) __ptr64` |
| 251 | 0x00026450 | `public: virtual void __cdecl URedRoadsSegmentComponent::OnRegister(void) __ptr64` |
| 252 | 0x000233E0 | `public: virtual void __cdecl URedRoadsJunctionComponent::OnUnregister(void) __ptr64` |
| 253 | 0x000121B0 | `private: virtual void __cdecl URedRoadsLandscapeComponent::OnUnregister(void) __ptr64` |
| 254 | 0x00012360 | `private: virtual void __cdecl URedRoadsMeshComponent::OnUnregister(void) __ptr64` |
| 255 | 0x00026660 | `public: virtual void __cdecl URedRoadsSegmentComponent::OnUnregister(void) __ptr64` |
| 256 | 0x000125A0 | `public: virtual void __cdecl ARedRoadsJunction::PostActorCreated(void) __ptr64` |
| 257 | 0x00012700 | `public: virtual void __cdecl ARedRoadsSegment::PostActorCreated(void) __ptr64` |
| 258 | 0x000128B0 | `public: virtual void __cdecl URedRoadsElementComponent::PostEditChangeProperty(struct FPropertyChangedEvent & __ptr64) __ptr64` |
| 259 | 0x00026810 | `public: virtual void __cdecl URedRoadsSegmentComponent::PostEditChangeProperty(struct FPropertyChangedEvent & __ptr64) __ptr64` |
| 260 | 0x00023590 | `public: virtual void __cdecl URedRoadsJunctionComponent::PostLoad(void) __ptr64` |
| 261 | 0x00026820 | `public: virtual void __cdecl URedRoadsSegmentComponent::PostLoad(void) __ptr64` |
| 262 | 0x0006BDF0 | `public: static class URedRoadsSettings const * __ptr64 const __ptr64 FRedRoadsModule::Settings` |
| 263 | 0x000152A0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowBounds(void)const __ptr64` |
| 264 | 0x000152D0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowControlPointDirection(void)const __ptr64` |
| 265 | 0x00015300 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowControlPointIndices(void)const __ptr64` |
| 266 | 0x00015330 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowControlPointsIfDeselected(void)const __ptr64` |
| 267 | 0x00015350 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowDirectionIndicators(void)const __ptr64` |
| 268 | 0x00015360 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowFalloffLines(void)const __ptr64` |
| 269 | 0x00015370 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowInfoMesh(void)const __ptr64` |
| 270 | 0x00015390 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowInfoMeshDebug(void)const __ptr64` |
| 271 | 0x000153C0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowLeftFalloffLines(void)const __ptr64` |
| 272 | 0x000153D0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowLeftFalloffWidthHandles(void)const __ptr64` |
| 273 | 0x000153E0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowMiddleSpline(void)const __ptr64` |
| 274 | 0x00015410 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowRightFalloffLine(void)const __ptr64` |
| 275 | 0x00015430 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowRightFalloffWidthHandles(void)const __ptr64` |
| 276 | 0x00015460 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowSamples(void)const __ptr64` |
| 277 | 0x00015490 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowSelectedControlPointHandles(void)const __ptr64` |
| 278 | 0x000154B0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowSideLines(void)const __ptr64` |
| 279 | 0x000154E0 | `public: virtual bool __cdecl URedRoadsSplineNew::ShouldShowWidthHandles(void)const __ptr64` |
| 280 | 0x00013140 | `public: virtual void __cdecl FRedRoadsModule::ShutdownModule(void) __ptr64` |
| 281 | 0x00024880 | `public: virtual void __cdecl URedRoadsJunctionComponent::SnapToLandscape(class TArray&lt;class URedUniversalSplineControlPoint * __ptr64,class TSizedDefaultAllocator&lt;32&gt; &gt; & __ptr64) __ptr64` |
| 282 | 0x00027BA0 | `public: virtual void __cdecl URedRoadsSegmentComponent::SnapToLandscape(class TArray&lt;class URedUniversalSplineControlPoint * __ptr64,class TSizedDefaultAllocator&lt;32&gt; &gt; & __ptr64) __ptr64` |
| 283 | 0x00013160 | `public: virtual void __cdecl FRedRoadsModule::StartupModule(void) __ptr64` |
| 284 | 0x00013320 | `public: static class UClass * __ptr64 __cdecl ARedRoadsElement::StaticClass(void)` |
| 285 | 0x00013330 | `public: static class UClass * __ptr64 __cdecl ARedRoadsJunction::StaticClass(void)` |
| 286 | 0x00013340 | `public: static class UClass * __ptr64 __cdecl ARedRoadsSegment::StaticClass(void)` |
| 287 | 0x00013370 | `public: static class UClass * __ptr64 __cdecl URedRoadsElementComponent::StaticClass(void)` |
| 288 | 0x00013380 | `public: static class UClass * __ptr64 __cdecl URedRoadsJunctionComponent::StaticClass(void)` |
| 289 | 0x00013390 | `public: static class UClass * __ptr64 __cdecl URedRoadsLandscapeComponent::StaticClass(void)` |
| 290 | 0x000133A0 | `public: static class UClass * __ptr64 __cdecl URedRoadsMeshComponent::StaticClass(void)` |
| 291 | 0x000133B0 | `public: static class UClass * __ptr64 __cdecl URedRoadsPreset::StaticClass(void)` |
| 292 | 0x000133C0 | `public: static class UClass * __ptr64 __cdecl URedRoadsProjectSettings::StaticClass(void)` |
| 293 | 0x000133D0 | `public: static class UClass * __ptr64 __cdecl URedRoadsRVTDecalComponent::StaticClass(void)` |
| 294 | 0x000133E0 | `public: static class UClass * __ptr64 __cdecl URedRoadsSegmentComponent::StaticClass(void)` |
| 295 | 0x000133F0 | `public: static class UClass * __ptr64 __cdecl URedRoadsSettings::StaticClass(void)` |
| 296 | 0x00013400 | `public: static class UClass * __ptr64 __cdecl URedRoadsSplineControlPoint::StaticClass(void)` |
| 297 | 0x00013410 | `public: static class UClass * __ptr64 __cdecl URedRoadsSplineNew::StaticClass(void)` |
| 298 | 0x00013420 | `public: static class UClass * __ptr64 __cdecl URedRoadsStaticRVTDecalComponent::StaticClass(void)` |
| 299 | 0x00013430 | `public: static enum EClassCastFlags __cdecl ARedRoadsElement::StaticClassCastFlags(void)` |
| 300 | 0x00013440 | `public: static enum EClassCastFlags __cdecl ARedRoadsJunction::StaticClassCastFlags(void)` |
| 301 | 0x00013450 | `public: static enum EClassCastFlags __cdecl ARedRoadsSegment::StaticClassCastFlags(void)` |
| 302 | 0x00013460 | `public: static enum EClassCastFlags __cdecl URedRoadsElementComponent::StaticClassCastFlags(void)` |
| 303 | 0x00013470 | `public: static enum EClassCastFlags __cdecl URedRoadsJunctionComponent::StaticClassCastFlags(void)` |
| 304 | 0x00013480 | `public: static enum EClassCastFlags __cdecl URedRoadsLandscapeComponent::StaticClassCastFlags(void)` |
| 305 | 0x00013490 | `public: static enum EClassCastFlags __cdecl URedRoadsMeshComponent::StaticClassCastFlags(void)` |
| 306 | 0x000134A0 | `public: static enum EClassCastFlags __cdecl URedRoadsPreset::StaticClassCastFlags(void)` |
| 307 | 0x000134B0 | `public: static enum EClassCastFlags __cdecl URedRoadsProjectSettings::StaticClassCastFlags(void)` |
| 308 | 0x000134C0 | `public: static enum EClassCastFlags __cdecl URedRoadsRVTDecalComponent::StaticClassCastFlags(void)` |
| 309 | 0x000134D0 | `public: static enum EClassCastFlags __cdecl URedRoadsSegmentComponent::StaticClassCastFlags(void)` |
| 310 | 0x000134E0 | `public: static enum EClassCastFlags __cdecl URedRoadsSettings::StaticClassCastFlags(void)` |
| 311 | 0x000134F0 | `public: static enum EClassCastFlags __cdecl URedRoadsSplineControlPoint::StaticClassCastFlags(void)` |
| 312 | 0x00013500 | `public: static enum EClassCastFlags __cdecl URedRoadsSplineNew::StaticClassCastFlags(void)` |
| 313 | 0x00013510 | `public: static enum EClassCastFlags __cdecl URedRoadsStaticRVTDecalComponent::StaticClassCastFlags(void)` |
| 314 | 0x00038A38 | `public: static enum EClassFlags const ARedRoadsElement::StaticClassFlags` |
| 315 | 0x000349FC | `public: static enum EClassFlags const ARedRoadsJunction::StaticClassFlags` |
| 316 | 0x00036780 | `public: static enum EClassFlags const ARedRoadsSegment::StaticClassFlags` |
| 317 | 0x000346A8 | `public: static enum EClassFlags const URedRoadsElementComponent::StaticClassFlags` |
| 318 | 0x00034D0C | `public: static enum EClassFlags const URedRoadsJunctionComponent::StaticClassFlags` |
| 319 | 0x000355A8 | `public: static enum EClassFlags const URedRoadsLandscapeComponent::StaticClassFlags` |
| 320 | 0x00034D10 | `public: static enum EClassFlags const URedRoadsMeshComponent::StaticClassFlags` |
| 321 | 0x00034B2C | `public: static enum EClassFlags const URedRoadsPreset::StaticClassFlags` |
| 322 | 0x00034BE8 | `public: static enum EClassFlags const URedRoadsProjectSettings::StaticClassFlags` |
| 323 | 0x00035B0C | `public: static enum EClassFlags const URedRoadsRVTDecalComponent::StaticClassFlags` |
| 324 | 0x000368F4 | `public: static enum EClassFlags const URedRoadsSegmentComponent::StaticClassFlags` |
| 325 | 0x00034B44 | `public: static enum EClassFlags const URedRoadsSettings::StaticClassFlags` |
| 326 | 0x00035AA0 | `public: static enum EClassFlags const URedRoadsSplineControlPoint::StaticClassFlags` |
| 327 | 0x00035AFC | `public: static enum EClassFlags const URedRoadsSplineNew::StaticClassFlags` |
| 328 | 0x00039FC8 | `public: static enum EClassFlags const URedRoadsStaticRVTDecalComponent::StaticClassFlags` |
| 329 | 0x00013520 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsProjectSettings::StaticConfigName(void)` |
| 330 | 0x00013530 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsSettings::StaticConfigName(void)` |
| 331 | 0x00013540 | `public: static wchar_t const * __ptr64 __cdecl ARedRoadsElement::StaticPackage(void)` |
| 332 | 0x00013550 | `public: static wchar_t const * __ptr64 __cdecl ARedRoadsJunction::StaticPackage(void)` |
| 333 | 0x00013560 | `public: static wchar_t const * __ptr64 __cdecl ARedRoadsSegment::StaticPackage(void)` |
| 334 | 0x00013570 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsElementComponent::StaticPackage(void)` |
| 335 | 0x00013580 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsJunctionComponent::StaticPackage(void)` |
| 336 | 0x00013590 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsLandscapeComponent::StaticPackage(void)` |
| 337 | 0x000135A0 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsMeshComponent::StaticPackage(void)` |
| 338 | 0x000135B0 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsPreset::StaticPackage(void)` |
| 339 | 0x000135C0 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsProjectSettings::StaticPackage(void)` |
| 340 | 0x000135D0 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsRVTDecalComponent::StaticPackage(void)` |
| 341 | 0x000135E0 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsSegmentComponent::StaticPackage(void)` |
| 342 | 0x000135F0 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsSettings::StaticPackage(void)` |
| 343 | 0x00013600 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsSplineControlPoint::StaticPackage(void)` |
| 344 | 0x00013610 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsSplineNew::StaticPackage(void)` |
| 345 | 0x00013620 | `public: static wchar_t const * __ptr64 __cdecl URedRoadsStaticRVTDecalComponent::StaticPackage(void)` |
| 346 | 0x00013630 | `public: static class UScriptStruct * __ptr64 __cdecl FRedRoadsControlPoint::StaticStruct(void)` |
| 347 | 0x00013640 | `public: static class UScriptStruct * __ptr64 __cdecl FRedRoadsEnd::StaticStruct(void)` |
| 348 | 0x00013650 | `public: static class UScriptStruct * __ptr64 __cdecl FRedRoadsSpline::StaticStruct(void)` |
| 349 | 0x00013660 | `public: static class UScriptStruct * __ptr64 __cdecl FRedRoadsTier::StaticStruct(void)` |
| 350 | 0x0001FA00 | `bool __cdecl RedRoadsUtils::TrySnapRoadsSegmentControlPointToNearbyRoadsJunctionSide(class URedUniversalSplineControlPoint * __ptr64,struct FJunctionSideSplinePointData & __ptr64,bool,bool)` |
| 351 | 0x00025030 | `public: virtual void __cdecl URedRoadsJunctionComponent::Update(bool) __ptr64` |
| 352 | 0x00027D10 | `public: virtual void __cdecl URedRoadsSegmentComponent::Update(bool) __ptr64` |
| 353 | 0x000250E0 | `public: void __cdecl URedRoadsJunctionComponent::UpdateConnections(void) __ptr64` |
| 354 | 0x00027DC0 | `public: void __cdecl URedRoadsSegmentComponent::UpdateConnections(bool) __ptr64` |
| 355 | 0x00007740 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_ARedRoadsElement(enum ETypeConstructPhase)` |
| 356 | 0x00005500 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_ARedRoadsJunction(enum ETypeConstructPhase)` |
| 357 | 0x00006F80 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_ARedRoadsSegment(enum ETypeConstructPhase)` |
| 358 | 0x000052D0 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsElementComponent(enum ETypeConstructPhase)` |
| 359 | 0x00005DC0 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsJunctionComponent(enum ETypeConstructPhase)` |
| 360 | 0x00006250 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsLandscapeComponent(enum ETypeConstructPhase)` |
| 361 | 0x00005FF0 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsMeshComponent(enum ETypeConstructPhase)` |
| 362 | 0x00005730 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsPreset(enum ETypeConstructPhase)` |
| 363 | 0x00005B90 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsProjectSettings(enum ETypeConstructPhase)` |
| 364 | 0x00006C20 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsRVTDecalComponent(enum ETypeConstructPhase)` |
| 365 | 0x000072E0 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsSegmentComponent(enum ETypeConstructPhase)` |
| 366 | 0x00005960 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsSettings(enum ETypeConstructPhase)` |
| 367 | 0x00006480 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsSplineControlPoint(enum ETypeConstructPhase)` |
| 368 | 0x000066B0 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsSplineNew(enum ETypeConstructPhase)` |
| 369 | 0x00007E60 | `class UClass * __ptr64 __cdecl Z_Construct_UClass_URedRoadsStaticRVTDecalComponent(enum ETypeConstructPhase)` |
| 370 | 0x000080C0 | `class UEnum * __ptr64 __cdecl Z_Construct_UEnum_RedRoads_ERedRoadTier(enum ETypeConstructPhase)` |
| 371 | 0x00007A30 | `class UEnum * __ptr64 __cdecl Z_Construct_UEnum_RedRoads_ERedRoadsEndType(enum ETypeConstructPhase)` |
| 372 | 0x00006B50 | `class UScriptStruct * __ptr64 __cdecl Z_Construct_UScriptStruct_FJunctionToSegmentConnection(enum ETypeConstructPhase)` |
| 373 | 0x000068E0 | `class UScriptStruct * __ptr64 __cdecl Z_Construct_UScriptStruct_FRedRoadsControlPoint(enum ETypeConstructPhase)` |
| 374 | 0x00006A80 | `class UScriptStruct * __ptr64 __cdecl Z_Construct_UScriptStruct_FRedRoadsEnd(enum ETypeConstructPhase)` |
| 375 | 0x000069B0 | `class UScriptStruct * __ptr64 __cdecl Z_Construct_UScriptStruct_FRedRoadsSpline(enum ETypeConstructPhase)` |
| 376 | 0x00007670 | `class UScriptStruct * __ptr64 __cdecl Z_Construct_UScriptStruct_FRedRoadsTier(enum ETypeConstructPhase)` |
| 377 | 0x00013BC0 | `public: static void __cdecl ARedRoadsElement::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 378 | 0x00013BF0 | `public: static void __cdecl ARedRoadsJunction::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 379 | 0x00013C10 | `public: static void __cdecl ARedRoadsSegment::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 380 | 0x00013C30 | `public: static void __cdecl URedRoadsElementComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 381 | 0x00013C50 | `public: static void __cdecl URedRoadsJunctionComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 382 | 0x00013C70 | `public: static void __cdecl URedRoadsLandscapeComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 383 | 0x00013CD0 | `public: static void __cdecl URedRoadsMeshComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 384 | 0x00013CF0 | `public: static void __cdecl URedRoadsPreset::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 385 | 0x00013D20 | `public: static void __cdecl URedRoadsProjectSettings::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 386 | 0x00013D60 | `public: static void __cdecl URedRoadsRVTDecalComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 387 | 0x00013DC0 | `public: static void __cdecl URedRoadsSegmentComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 388 | 0x00013DE0 | `public: static void __cdecl URedRoadsSettings::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 389 | 0x00013E00 | `public: static void __cdecl URedRoadsSplineControlPoint::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 390 | 0x00013E30 | `public: static void __cdecl URedRoadsSplineNew::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 391 | 0x00013E70 | `public: static void __cdecl URedRoadsStaticRVTDecalComponent::__DefaultConstructor(class FObjectInitializer const & __ptr64)` |
| 392 | 0x00013E90 | `public: static class UObject * __ptr64 __cdecl ARedRoadsElement::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 393 | 0x00013F50 | `public: static class UObject * __ptr64 __cdecl ARedRoadsJunction::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 394 | 0x00014010 | `public: static class UObject * __ptr64 __cdecl ARedRoadsSegment::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 395 | 0x000140D0 | `public: static class UObject * __ptr64 __cdecl URedRoadsElementComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 396 | 0x00014180 | `public: static class UObject * __ptr64 __cdecl URedRoadsJunctionComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 397 | 0x00014230 | `public: static class UObject * __ptr64 __cdecl URedRoadsLandscapeComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 398 | 0x00014320 | `public: static class UObject * __ptr64 __cdecl URedRoadsMeshComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 399 | 0x000143D0 | `public: static class UObject * __ptr64 __cdecl URedRoadsPreset::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 400 | 0x00014490 | `public: static class UObject * __ptr64 __cdecl URedRoadsProjectSettings::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 401 | 0x00014560 | `public: static class UObject * __ptr64 __cdecl URedRoadsRVTDecalComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 402 | 0x00014650 | `public: static class UObject * __ptr64 __cdecl URedRoadsSegmentComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 403 | 0x00014700 | `public: static class UObject * __ptr64 __cdecl URedRoadsSettings::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 404 | 0x000147B0 | `public: static class UObject * __ptr64 __cdecl URedRoadsSplineControlPoint::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 405 | 0x00014870 | `public: static class UObject * __ptr64 __cdecl URedRoadsSplineNew::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 406 | 0x00014940 | `public: static class UObject * __ptr64 __cdecl URedRoadsStaticRVTDecalComponent::__VTableCtorCaller(class FVTableHelper & __ptr64)` |
| 407 | 0x00014BB0 | `ThisIsAnUnrealEngineModule` |

</details>

## Debug / PDB (3 entries)

- **Format**: PDB70/RSDS  **GUID**: `C0E3EBAE-361C-4EE8-8A7E-26F0E1AB08DD`  **Age**: 1
- **PDB path**: `UnrealEditor-RedRoads.pdb`
- Type 12 entry, 20 bytes
- Type 13 entry, 940 bytes

## TLS

- **Callback count**: 1

## Load Config

- **Security cookie**: 0x18006BC80   **SEHandlerCount**: 0
- **Guard flags**: 0x00000100   **CFG instrumented**: True

## Signature / .NET

- **Authenticode signature present**: False
- **.NET / CLR (mixed-mode) assembly**: False

## Version Info

- **file_version**: 5.8.0.0
- **product_version**: 5.8.0.0
- **FileVersion**: 5.8.0
- **CompanyName**: Epic Games, Inc.
- **LegalCopyright**: Copyright Epic Games, Inc. All Rights Reserved.
- **ProductName**: UnrealEditor
- **ProductVersion**: 5.8.0
- **FileDescription**: UnrealEditor
- **InternalName**: UnrealEngine

## Rich Header (12 tool entries, key 0xD4A04F21)

| ProdId | BuildId | Count |
|---|---|---|
| 147 | 30729 | 6 |
| 257 | 35207 | 4 |
| 259 | 35207 | 4 |
| 260 | 35207 | 8 |
| 261 | 35207 | 17 |
| 257 | 30795 | 2 |
| 257 | 35228 | 15 |
| 1 | 0 | 1180 |
| 261 | 35228 | 5 |
| 255 | 35228 | 1 |
| 256 | 35228 | 1 |
| 258 | 35228 | 1 |

## Heuristics

- ⚠ No Control Flow Guard (GUARD_CF not set).
- ⚠ Not digitally signed (no Authenticode certificate).
- ⚠ 1 TLS callback(s) present -- these run before DllMain/StartupModule and can add to load latency.
- ⚠ Large export table: 407 symbols ('URedRoadsSplineNew' alone contributes 41).
