# KYC Integration Architecture for CPP Platform (Motoko Canisters)

## Overview

This document outlines the **core KYC canister architecture** implemented in **Motoko** for the Cool Planet Platform (CPP). The implementation focuses on high-performance, secure KYC data storage, risk assessment, and compliance management using IC-native Motoko canisters.

## Authoritative References

### Risk Assessment & Compliance Framework
- **Primary Source**: [`cpf_compliance_documentation/financial/kyc-trigger-policy-matrix.md`](../../../cpf_compliance_documentation/financial/kyc-trigger-policy-matrix.md)
  - Transaction amount thresholds (€0-€250, €251-€999, €1,000-€4,999, etc.)
  - Jurisdiction risk classifications (Low, Medium, High, Prohibited)
  - Social verification requirements and methods
  - Enhanced Due Diligence (EDD) triggers

### KYC/AML Policy Framework
- **Primary Source**: [`cpf_compliance_documentation/financial/kyc-aml-policy.md`](../../../cpf_compliance_documentation/financial/kyc-aml-policy.md)
  - Risk-based approach and categories
  - Customer due diligence procedures
  - Sanctions and watchlist screening
  - Monitoring and reporting requirements

### Zero-Knowledge Proof Implementation
- **Primary Source**: [`einstein_solidity/docs/src/kyc.md`](../../../einstein_solidity/docs/src/kyc.md)
  - ZK proof generation and verification
  - Privacy-preserving verification approach
  - Multi-provider integration (Jumio, Onfido, Coinbase)
  - Audit trail management

### KYC-Light Verification Options
- **LinkedIn OAuth**: Social verification for professional identity
- **Gravatar Integration**: Email-verified identity linking and profile verification
- **Risk-based selection**: Automatic provider selection based on donation amount and jurisdiction

### KYC Types and Risk Levels
- **Primary Source**: [`cpf_compliance_documentation/financial/kyc-aml-policy.md`](../../../cpf_compliance_documentation/financial/kyc-aml-policy.md)
  - Risk categories (Low, Medium, High)
  - Customer due diligence procedures by risk level
  - Verification requirements and documentation standards
  - Enhanced Due Diligence (EDD) procedures

## Motoko Canister Architecture

### **Core KYC Canisters**

```motoko
// Main KYC canister types
type KYCCanister = {
    // KYC Storage Canister - Jurisdiction-specific data storage
    kycStorage: KYCStorageCanister;
    
    // Risk Assessment Canister - Real-time risk calculation
    riskAssessment: RiskAssessmentCanister;
    
    // Threshold Management Canister - Updatable configuration
    thresholdManagement: ThresholdManagementCanister;
    
    // Audit Trail Canister - Compliance and audit logging
    auditTrail: AuditTrailCanister;
    
    // Jurisdiction Detection Canister - IP-based location detection
    jurisdictionDetection: JurisdictionDetectionCanister;
};
```

### **1. KYC Storage Canister (Motoko)**

```motoko
import HashMap "mo:base/HashMap";
import Text "mo:base/Text";
import Principal "mo:base/Principal";
import Time "mo:base/Time";
import Array "mo:base/Array";
import Result "mo:base/Result";

actor KYCStorageCanister {
    
    // Types
    type KYCData = {
        userDID: Text;
        jurisdiction: Text;
        encryptedData: Blob;
        dataClassification: Text; // "high", "medium", "low"
        retentionPeriod: Int;
        encryptionLevel: Text;
        auditRequired: Bool;
        timestamp: Int;
    };
    
    type StorageResult = {
        success: Bool;
        canisterId: Text;
        storageKey: Text;
        timestamp: Int;
    };
    
    // Stable storage for jurisdiction-specific canisters
    private stable var jurisdictionCanisters: [(Text, Principal)] = [];
    private var jurisdictionCanisterMap = HashMap.HashMap<Text, Principal>(0, Text.equal, Text.hash);
    
    // System upgrade hooks
    system func preupgrade() {
        jurisdictionCanisters := Array.map<(Text, Principal), (Text, Principal)>(
            jurisdictionCanisterMap.entries() |> Array.fromIter,
            func(entry) = entry
        );
    };
    
    system func postupgrade() {
        jurisdictionCanisters := [];
    };
    
    // Store KYC data in jurisdiction-specific canister
    public func storeKYCData(userDID: Text, kycData: KYCData, jurisdiction: Text): async Result.Result<StorageResult, Text> {
        // Get or create jurisdiction-specific canister
        let canister = await getJurisdictionCanister(jurisdiction);
        
        // Store encrypted data
        let storageResult = await canister.store(userDID, kycData);
        
        #ok({
            success = true;
            canisterId = Principal.toText(canister);
            storageKey = userDID;
            timestamp = Time.now();
        })
    };
    
    // Retrieve KYC data with proper authorization
    public func retrieveKYCData(userDID: Text, jurisdiction: Text, authorization: Authorization): async Result.Result<KYCData, Text> {
        let canister = await getJurisdictionCanister(jurisdiction);
        let encryptedData = await canister.retrieve(userDID);
        
        // Decrypt data based on jurisdiction requirements
        let decryptedData = await decryptKYCData(encryptedData, jurisdiction);
        
        #ok(decryptedData)
    };
    
    private func getJurisdictionCanister(jurisdiction: Text): async Principal {
        switch (jurisdictionCanisterMap.get(jurisdiction)) {
            case (?canister) { canister };
            case null {
                // Create new jurisdiction-specific canister
                let newCanister = await createJurisdictionCanister(jurisdiction);
                jurisdictionCanisterMap.put(jurisdiction, newCanister);
                newCanister
            };
        }
    };
}
```

### **2. Risk Assessment Canister (Motoko)**

```motoko
actor RiskAssessmentCanister {
    
    // Types
    type RiskAssessment = {
        riskLevel: Text; // "low", "medium", "high", "critical"
        kycRequired: Bool;
        requiredLevel: Text;
        processingTime: Int;
        additionalChecks: [Text];
        confidence: Float;
    };
    
    type AmountRisk = {
        level: Text;
        kycRequired: Bool;
        socialVerification: Bool;
        enhancedChecks: Bool;
        eddRequired: Bool;
        boardApproval: Bool;
    };
    
    // Threshold table (updatable by canister owner)
    private stable var thresholds: Thresholds = defaultThresholds();
    
    // Threshold types for configuration
    type AmountThreshold = {
        maxAmount: Nat;
        riskLevel: Text;
        kycRequired: Bool;
        socialVerification: Bool;
        enhancedChecks: Bool;
        eddRequired: Bool;
        boardApproval: Bool;
    };
    
    type Thresholds = {
        amountThresholds: [AmountThreshold];
        jurisdictionRiskLevels: [(Text, Text)]; // (jurisdiction, risk_level)
        cumulativeThresholds: [Nat];
        socialVerificationMethods: [Text]; // "linkedin", "gravatar", "full_kyc"
    };
    
    // Assess risk for donation
    public func assessRisk(donationAmount: Nat, jurisdiction: Text, userDID: Text): async RiskAssessment {
        // 1. Determine amount-based risk
        let amountRisk = calculateAmountRisk(donationAmount);
        
        // 2. Determine jurisdiction risk
        let jurisdictionRisk = calculateJurisdictionRisk(jurisdiction);
        
        // 3. Check cumulative risk
        let cumulativeRisk = await calculateCumulativeRisk(userDID);
        
        // 4. Combine risk factors
        let finalRisk = combineRiskFactors(amountRisk, jurisdictionRisk, cumulativeRisk);
        
        finalRisk
    };
    
    private func calculateAmountRisk(amount: Nat): AmountRisk {
        // Query thresholds from updatable configuration
        let amountThresholds = thresholds.amountThresholds;
        
        // Find the appropriate threshold for the donation amount
        for (threshold in amountThresholds.vals()) {
            if (amount <= threshold.maxAmount) {
                return {
                    level = threshold.riskLevel;
                    kycRequired = threshold.kycRequired;
                    socialVerification = threshold.socialVerification;
                    enhancedChecks = threshold.enhancedChecks;
                    eddRequired = threshold.eddRequired;
                    boardApproval = threshold.boardApproval
                };
            };
        };
        
        // Default to maximum risk if amount exceeds all thresholds
        {
            level = "maximum";
            kycRequired = true;
            socialVerification = false;
            enhancedChecks = true;
            eddRequired = true;
            boardApproval = true
        }
    };
    
    // Canister owner can update thresholds
    public shared({caller}) func updateThresholds(newThresholds: Thresholds): async Result.Result<(), Text> {
        if (not Principal.isController(caller)) {
            return #err("Only canister owner can update thresholds");
        };
        
        thresholds := newThresholds;
        
        // Log the update for audit trail
        await auditTrailCanister.logThresholdUpdate(newThresholds);
        
        #ok(())
    };
}

## Consent Management & User Choice

### **Consent Basis Before KYC**

**Legal Basis**: GDPR Article 6(1)(b) - Contract performance for donation processing
**Required Consent Elements**:
1. **Purpose**: KYC verification for donation processing and regulatory compliance
2. **Methods**: Available verification options and their requirements
3. **Data Processing**: How personal data will be used and stored
4. **Rights**: User rights under GDPR (access, rectification, deletion)
5. **Duration**: How long data will be retained
6. **Third Parties**: Which external providers will process data

**Consent Collection Process**:
```motoko
type ConsentRecord = {
    userDID: Text;
    consentTimestamp: Int;
    consentVersion: Text;
    verificationMethods: [Text]; // "gravatar", "linkedin", "full_kyc"
    dataProcessingPurposes: [Text];
    retentionPeriod: Int;
    userRights: [Text];
    consentStatus: Text; // "active", "withdrawn", "expired"
};
```

### **User Choice in KYC Methods**

**System Recommendation**: Based on risk assessment, system suggests optimal verification method
**User Override**: Users can choose alternative verification methods that satisfy risk requirements
**Method Equivalence**: Different methods provide equivalent verification levels for same risk tier

**Available Verification Methods by Risk Level**:

| Risk Level             | Required Verification | Alternative Options            | User Choice                |
| ---------------------- | --------------------- | ------------------------------ | -------------------------- |
| **Low (€0-€250)**      | Gravatar + Email      | LinkedIn OAuth                 | User can choose either     |
| **Medium (€251-€999)** | LinkedIn OAuth        | Full KYC                       | User can choose either     |
| **High (€1,000+)**     | Full KYC              | Enhanced LinkedIn + Additional | User can choose equivalent |

### **Progressive KYC Enhancement**

**Initial Donation**: User completes minimum required verification for donation amount
**Post-Donation Enhancement**: Users can voluntarily upgrade verification for:
- **Higher donation limits**: Unlock larger donation capabilities
- **Enhanced ZK Proof**: More comprehensive DID verification
- **Community privileges**: Access to exclusive community features
- **Future compliance**: Prepare for regulatory changes

**ZK Proof Enhancement Workflow**:
1. **Initial ZK Proof**: Generated from minimum required verification
2. **User Choice**: User decides to enhance verification
3. **Additional Verification**: Complete higher-level KYC method
4. **Enhanced ZK Proof**: Updated proof with additional verification data
5. **DID Update**: Enhanced proof stored in Polygon DID registry

### **Gravatar KYC-Light Verification**

**Use Case**: Low-risk donations (€0-€250) where email is already verified
**Verification Process**:
1. **Consent Collection**: User consents to Gravatar verification
2. **Email Retrieval**: Get email from Gravatar API (not II verification)
3. **Profile Verification**: Confirm Gravatar profile exists and is active
4. **Identity Confirmation**: Verify email matches user's claimed identity
5. **Risk Assessment**: Lower risk score due to verified email + public profile
6. **ZK Proof Generation**: Create privacy-preserving verification proof

**Benefits**:
- **Reduced friction**: No document upload required
- **Email verification**: Leverages existing II email verification
- **Public profile**: Gravatar provides additional identity confidence
- **Privacy-preserving**: ZK proof maintains user privacy
- **Cost-effective**: No external KYC provider fees

**Risk Considerations**:
- **Limited verification**: No government ID verification
- **Email-based**: Relies on email verification accuracy
- **Public data**: Gravatar data is publicly accessible
- **Suitable for**: Low-risk transactions and community interactions

### **Consent & Verification Flow**

**Step 1: Risk Assessment & Method Recommendation**
```motoko
public func recommendVerificationMethod(donationAmount: Nat, jurisdiction: Text): async VerificationRecommendation {
    let riskAssessment = await assessRisk(donationAmount, jurisdiction);
    let recommendedMethod = getRecommendedMethod(riskAssessment);
    let alternativeMethods = getAlternativeMethods(riskAssessment);
    
    {
        recommendedMethod = recommendedMethod;
        alternativeMethods = alternativeMethods;
        riskLevel = riskAssessment.riskLevel;
        explanation = getMethodExplanation(recommendedMethod);
    }
}
```

**Step 2: User Consent Collection**
```motoko
public func collectConsent(userDID: Text, verificationMethod: Text, consentData: ConsentData): async ConsentRecord {
    let consentRecord = {
        userDID = userDID;
        consentTimestamp = Time.now();
        consentVersion = "1.0";
        verificationMethods = [verificationMethod];
        dataProcessingPurposes = consentData.purposes;
        retentionPeriod = consentData.retentionPeriod;
        userRights = consentData.userRights;
        consentStatus = "active";
    };
    
    await consentCanister.storeConsent(consentRecord);
    consentRecord
}
```

**Step 3: Method-Specific Verification**
- **Gravatar**: Retrieve email from Gravatar API, verify profile
- **LinkedIn**: OAuth flow, verify professional identity
- **Full KYC**: External provider integration, document verification

**Step 4: ZK Proof Generation & DID Storage**
```motoko
public func generateZKProof(verificationResult: VerificationResult): async ZKProof {
    let zkProof = await zkProofCanister.generateProof(verificationResult);
    await didRegistryCanister.storeProof(zkProof);
    zkProof
}
```

**Step 5: Post-Donation Enhancement Options**
```motoko
public func offerEnhancement(userDID: Text): async EnhancementOptions {
    let currentProof = await didRegistryCanister.getProof(userDID);
    let enhancementOptions = getEnhancementOptions(currentProof);
    
    {
        currentLevel = currentProof.verificationLevel;
        availableUpgrades = enhancementOptions;
        benefits = getEnhancementBenefits(enhancementOptions);
    }
}
```
```

### **3. Jurisdiction Detection Canister (Motoko)**

```motoko
actor JurisdictionDetectionCanister {
    
    // Types
    type JurisdictionDetection = {
        systemDetectedJurisdiction: Text;
        confidence: Float;
        riskFactors: [Text];
        requiresUserConfirmation: Bool;
        suggestedJurisdiction: Text;
        detectionMethod: Text;
        timestamp: Int;
        complianceNote: Text;
    };
    
    type UserJurisdictionConfirmation = {
        userDID: Text;
        userConfirmedJurisdiction: Text;
        systemDetectedJurisdiction: Text;
        confirmationTimestamp: Int;
        userAgent: Text;
        ipAddress: Text;
        riskFactors: [Text];
        complianceNote: Text;
    };
    
    // Detect jurisdiction from IP and network analysis
    public func detectJurisdiction(userIP: Text, userAgent: Text): async JurisdictionDetection {
        // 1. IP-based geolocation
        let ipLocation = await analyzeIPLocation(userIP);
        
        // 2. Network analysis (VPN detection, proxy detection)
        let networkAnalysis = await analyzeNetwork(userIP, userAgent);
        
        // 3. Cross-reference with stored jurisdiction data
        let storedJurisdiction = await getStoredJurisdiction(userIP);
        
        // 4. Calculate confidence score
        let confidence = calculateConfidence(ipLocation, networkAnalysis, storedJurisdiction);
        
        {
            systemDetectedJurisdiction = ipLocation.country;
            confidence = confidence;
            riskFactors = networkAnalysis.riskFactors;
            requiresUserConfirmation = confidence < 0.8;
            suggestedJurisdiction = ipLocation.country;
            detectionMethod = "ip_geolocation";
            timestamp = Time.now();
            complianceNote = "System detection is for risk assessment only. User confirmation is authoritative for compliance.";
        }
    };
    
    // Store user's jurisdiction confirmation
    public func getUserConfirmedJurisdiction(userDID: Text, userConfirmedJurisdiction: Text): async UserJurisdictionConfirmation {
        let confirmation = {
            userDID = userDID;
            userConfirmedJurisdiction = userConfirmedJurisdiction;
            systemDetectedJurisdiction = await getSystemDetection(userDID);
            confirmationTimestamp = Time.now();
            userAgent = await getUserAgent(userDID);
            ipAddress = await getUserIP(userDID);
            riskFactors = await getRiskFactors(userDID);
            complianceNote = "User bears responsibility for accurate jurisdiction declaration";
        };
        
        // Store confirmation in audit canister
        await auditTrailCanister.storeJurisdictionConfirmation(confirmation);
        
        confirmation
    };
    
    private func calculateConfidence(ipLocation: IPLocation, networkAnalysis: NetworkAnalysis, storedJurisdiction: ?Text): Float {
        var confidence = 0.9; // Base confidence
        
        // Reduce confidence for suspicious network activity
        if (networkAnalysis.isVPN) { confidence -= 0.3 };
        if (networkAnalysis.isProxy) { confidence -= 0.2 };
        if (networkAnalysis.isTor) { confidence -= 0.4 };
        
        // Increase confidence if matches stored data
        switch (storedJurisdiction) {
            case (?stored) {
                if (stored == ipLocation.country) {
                    confidence += 0.1;
                };
            };
            case null { };
        };
        
        Float.max(0.1, Float.min(1.0, confidence))
    };
}
```

### **4. Audit Trail Canister (Motoko)**

```motoko
actor AuditTrailCanister {
    
    // Types
    type AuditRecord = {
        id: Text;
        userDID: Text;
        action: Text;
        timestamp: Int;
        jurisdiction: Text;
        metadata: ?Text;
        complianceLevel: Text;
    };
    
    // Stable storage for audit records
    private stable var auditRecords: [AuditRecord] = [];
    private var auditStore = HashMap.HashMap<Text, AuditRecord>(0, Text.equal, Text.hash);
    
    // Create audit trail entry
    public func createAuditTrail(userDID: Text, action: Text, jurisdiction: Text): async () {
        let record = {
            id = generateAuditId();
            userDID = userDID;
            action = action;
            timestamp = Time.now();
            jurisdiction = jurisdiction;
            metadata = null;
            complianceLevel = "standard";
        };
        
        auditStore.put(record.id, record);
    };
    
    // Store jurisdiction confirmation
    public func storeJurisdictionConfirmation(confirmation: UserJurisdictionConfirmation): async () {
        let record = {
            id = generateAuditId();
            userDID = confirmation.userDID;
            action = "jurisdiction_confirmation";
            timestamp = confirmation.confirmationTimestamp;
            jurisdiction = confirmation.userConfirmedJurisdiction;
            metadata = ?("System detected: " # confirmation.systemDetectedJurisdiction);
            complianceLevel = "high";
        };
        
        auditStore.put(record.id, record);
    };
    
    // Get audit trail for user
    public query func getAuditTrail(userDID: Text): async [AuditRecord] {
        let userRecords = Array.filter<AuditRecord>(
            auditStore.vals() |> Array.fromIter,
            func(record) = record.userDID == userDID
        );
        
        Array.sort<AuditRecord>(userRecords, func(a, b) = Int.compare(a.timestamp, b.timestamp))
    };
    
    private func generateAuditId(): Text {
        // Generate unique audit ID
        "audit-" # Int.toText(Time.now()) # "-" # Principal.toText(Principal.fromActor(AuditTrailCanister))
    };
    
    // System upgrade hooks
    system func preupgrade() {
        auditRecords := auditStore.vals() |> Array.fromIter;
    };
    
    system func postupgrade() {
        auditRecords := [];
    };
}
```

## Implementation Timeline

### **Phase 1: Core Motoko Canisters (Weeks 1-4)**
- [ ] Design and implement KYC Storage Canister
- [ ] Implement Risk Assessment Canister with updatable thresholds
- [ ] Create Jurisdiction Detection Canister with IP analysis
- [ ] Build Audit Trail Canister for compliance logging
- [ ] Set up inter-canister communication protocols
- [ ] Implement stable memory management for large datasets

### **Phase 2: Integration & Testing (Weeks 5-8)**
- [ ] Integrate with existing notification system (Motoko)
- [ ] Implement ZK proof generation and storage
- [ ] Set up jurisdiction-specific data residency
- [ ] Create comprehensive testing suite
- [ ] Performance optimization and memory management

### **Phase 3: Compliance & Security (Weeks 9-12)**
- [ ] GDPR compliance testing with IC storage
- [ ] Jurisdiction-specific compliance validation
- [ ] Security audit and penetration testing
- [ ] Regulatory reporting implementation
- [ ] Production deployment and monitoring

## Performance Metrics

### **Motoko Canister Performance**
- **KYC Data Storage**: Target <1 second for data storage/retrieval
- **Risk Assessment**: Target <2 seconds for complete risk evaluation
- **Jurisdiction Detection**: Target <1 second for IP analysis
- **Audit Trail Creation**: Target <500ms for audit record creation
- **Inter-Canister Calls**: Target <2 seconds for cross-canister communication
- **Memory Usage**: Target <100MB per jurisdiction canister
- **Stable Memory**: Efficient handling of large datasets across upgrades

## Success Metrics

### **Motoko-Specific Metrics**
- **Canister Performance**: Response times, memory utilization, cross-canister efficiency
- **Storage Efficiency**: Data compression, encryption overhead, retrieval speed
- **Jurisdiction Management**: Efficient jurisdiction-specific canister creation and management
- **Audit Trail Performance**: Fast audit record creation and retrieval
- **Threshold Updates**: Seamless threshold updates without canister redeployment
- **Memory Management**: Efficient stable memory usage across upgrades

### **Compliance Metrics**
- **Data Residency**: Proper jurisdiction-specific data storage
- **Audit Trail Integrity**: Complete and tamper-proof audit records
- **Regulatory Reporting**: Automated compliance reporting capabilities
- **Privacy Protection**: Secure encryption and data handling
- **Jurisdiction Compliance**: Proper handling of jurisdiction-specific requirements

## Conclusion

This Motoko-based KYC architecture provides high-performance, secure, and compliant KYC processing using IC-native canisters. The implementation leverages Motoko's strengths in memory efficiency, stable storage, and IC integration while maintaining full compliance with regulatory requirements.

For user interface and external integrations, see the companion document on TypeScript/Azle canister implementation. 