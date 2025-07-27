# Jitsi Video Conferencing Integration Architecture

**Date:** 2025-07-26  
**Purpose:** Video conferencing integration strategy for Cool Planet People community platform  
**Context:** Self-hosted video conferencing solution for CPP community meetings and educational content  
**Integration:** Internet Identity authentication and CPP community platform integration

**Cross-Reference:** See [Community Strategy](community-strategy.md) for overall community platform architecture and [Platform Comparison](platform-comparison.md) for authentication infrastructure analysis.

## Executive Summary

**RECOMMENDED APPROACH: Self-hosted Jitsi with Internet Identity Integration**

Jitsi provides a **vendor-independent video conferencing solution** that aligns perfectly with CPP's strategic goals. The integration leverages Internet Identity for seamless authentication while maintaining complete data sovereignty and eliminating vendor dependencies.

**Key Strategic Advantages:**
- **100% vendor independence** - self-hosted solution eliminates BigMES exposure
- **Internet Identity integration** - seamless authentication with CPP community platform
- **Swiss jurisdiction compliance** - complete data sovereignty when self-hosted
- **Cost-effective scaling** - no per-user licensing fees, predictable infrastructure costs
- **Community alignment** - open source solution matches CPP's community values
- **Feature completeness** - 90% of commercial platform capabilities

## Jitsi Platform Overview

### **Core Capabilities**

**Video Conferencing Features:**
- **HD Video**: Up to 720p/1080p video quality
- **Audio Conferencing**: High-quality audio with noise suppression
- **Screen Sharing**: Desktop, application, or browser tab sharing
- **File Sharing**: Real-time file transfer during meetings
- **Chat**: Text messaging during video calls
- **Reactions**: Emoji reactions and hand raising
- **Recording**: Local or cloud recording capabilities
- **Live Streaming**: Stream to YouTube, Facebook, etc.

**Collaboration Tools:**
- **Whiteboard**: Real-time collaborative whiteboarding
- **Document Sharing**: View and annotate documents together
- **Breakout Rooms**: Split large groups into smaller discussions
- **Custom Branding**: Complete UI customization
- **Analytics**: Meeting analytics and reporting

**Technical Features:**
- **WebRTC-based**: Modern web standards, no plugins required
- **REST API**: Full API for integration with CPP platform
- **Webhook Support**: Event notifications for meeting lifecycle
- **End-to-End Encryption**: Optional E2E encryption
- **Mobile Support**: Responsive design for mobile devices

### **Competitive Analysis**

| Feature                   | Jitsi | Zoom | Teams | Google Meet |
| ------------------------- | ----- | ---- | ----- | ----------- |
| **Video Quality**         | ✅ HD  | ✅ HD | ✅ HD  | ✅ HD        |
| **Screen Sharing**        | ✅     | ✅    | ✅     | ✅           |
| **Recording**             | ✅     | ✅    | ✅     | ✅           |
| **Chat**                  | ✅     | ✅    | ✅     | ✅           |
| **Breakout Rooms**        | ✅     | ✅    | ✅     | ✅           |
| **Whiteboard**            | ✅     | ✅    | ✅     | ✅           |
| **Live Streaming**        | ✅     | ✅    | ✅     | ✅           |
| **Mobile Apps**           | ✅     | ✅    | ✅     | ✅           |
| **End-to-End Encryption** | ✅     | ✅    | ✅     | ✅           |
| **Custom Branding**       | ✅     | ✅    | ✅     | ✅           |
| **Vendor Independence**   | ✅     | ❌    | ❌     | ❌           |
| **Data Sovereignty**      | ✅     | ❌    | ❌     | ❌           |
| **No Per-User Fees**      | ✅     | ❌    | ❌     | ❌           |
| **Open Source**           | ✅     | ❌    | ❌     | ❌           |

## Internet Identity Integration Architecture

### **Enterprise SSO Capabilities**

**Internet Identity (II) provides native enterprise SSO capabilities** that can directly integrate with Jitsi without requiring custom IdP development:

**Direct OIDC/SAML Integration:**
- **OIDC Provider**: II can act as a standard OpenID Connect provider
- **SAML Support**: Enterprise SAML integration capabilities  
- **Standard Protocols**: Compatible with enterprise SSO systems
- **No Custom IdP**: Eliminates need for intermediate authentication layer

### **Authentication Flow**

**Seamless CPP → Jitsi Authentication:**
```
CPP Community Platform → Internet Identity → Jitsi Meeting
├── User logs into CPP platform via Internet Identity
├── User clicks "Join Meeting" or "Create Meeting"
├── II provides OIDC tokens directly to Jitsi
├── Jitsi validates tokens and creates/joins meeting
├── User enters meeting with CPP identity preserved
└── Meeting analytics linked to CPP user account
```

**Technical Implementation:**
```javascript
// Direct II OIDC integration with Jitsi
const configureJitsiWithII = () => {
  const config = {
    hosts: {
      domain: 'meet.cpp-community.org',
      anonymousdomain: 'guest.cpp-community.org',
      muc: 'conference.meet.cpp-community.org'
    },
    externalConnectUrl: 'https://identity.internetcomputer.org',
    oidc: {
      clientId: 'cpp-jitsi-client',
      issuer: 'https://identity.internetcomputer.org',
      redirectUri: 'https://meet.cpp-community.org/oidc-callback',
      scope: 'openid profile email',
      responseType: 'code'
    },
    userInfo: {
      displayName: '${user.displayName}',
      email: '${user.email}',
      avatar: '${user.gravatarUrl}'
    }
  };
  
  return config;
};

// CPP Jitsi configuration with E2E option
const jitsiConfig = {
  roomName: 'cpp-community-meeting',
  configOverwrite: {
    // Enable E2E for sensitive meetings
    e2eEncryption: meetingType === 'governance' ? true : false,
    
    // Security settings
    securityOptions: {
      disableAudioLevels: true,
      disableSimulcast: false,
      enableLayerSuspension: true
    },
    
    // Privacy settings
    privacyOptions: {
      disableThirdPartyRequests: true,
      disableAnalytics: true,
      disableCrashReporting: true
    }
  }
};
```

### **Identity Management**

**Cool Planet ID Integration:**
- **Display Name**: Uses Cool Planet ID or Gravatar display name
- **Avatar**: Gravatar integration for visual identity
- **Email**: Internet Identity email for meeting invitations
- **Permissions**: Bundle sponsorship determines meeting access
- **Analytics**: Meeting participation linked to CPP reputation

**Privacy Considerations:**
- **GDPR Compliance**: Self-hosted solution ensures data sovereignty
- **Right to be Forgotten**: Meeting recordings can be deleted on request
- **Data Minimization**: Only necessary user data shared with Jitsi
- **Consent Management**: Clear consent for meeting recording and sharing

## Integration with CPP Community Platform

### **Community Event Integration**

**Meetup Calendar Integration:**
```javascript
// Automatic meeting creation from calendar events
const createMeetingFromEvent = (event) => {
  const meetingConfig = {
    roomName: `cpp-${event.id}-${event.date}`,
    subject: event.title,
    description: event.description,
    startTime: event.startTime,
    duration: event.duration,
    participants: event.attendees,
    accessLevel: event.accessLevel // public, members-only, sponsors-only
  };
  
  return jitsiAPI.createMeeting(meetingConfig);
};
```

**Event Management Features:**
- **Automatic Room Creation**: Meeting rooms created from calendar events
- **Access Control**: Bundle sponsorship determines meeting access
- **Invitation System**: Automatic invitations via CPP notification system
- **Recording Management**: Educational content preservation
- **Analytics Integration**: Meeting participation tracking

### **Educational Platform Integration**

**Course Delivery Features:**
- **Live Lectures**: Real-time video delivery for educational content
- **Office Hours**: One-on-one video sessions with instructors
- **Group Study**: Breakout room functionality for collaborative learning
- **Content Recording**: Course content preservation and replay
- **Interactive Whiteboard**: Real-time collaborative learning tools

**Learning Management Integration:**
```javascript
// Course session integration
const startCourseSession = (courseId, sessionId) => {
  const course = getCourse(courseId);
  const session = getSession(sessionId);
  
  const jitsiConfig = {
    roomName: `cpp-course-${courseId}-${sessionId}`,
    subject: `${course.title} - ${session.title}`,
    configOverwrite: {
      breakoutRooms: session.breakoutRooms,
      whiteboard: session.whiteboardEnabled,
      recording: session.recordingEnabled,
      chat: session.chatEnabled
    }
  };
  
  return jitsiAPI.createMeeting(jitsiConfig);
};
```

### **Community Governance Integration**

**Governance Meeting Features:**
- **Voting Integration**: Real-time voting during video meetings
- **Document Sharing**: Governance documents shared during meetings
- **Breakout Discussions**: Small group discussions for complex topics
- **Recording and Transcription**: Meeting minutes and action items
- **Access Control**: Role-based meeting access

## Technical Architecture

### **Self-Hosting Infrastructure**

**Recommended Deployment:**
```
Swiss Hosting Infrastructure:
├── Jitsi Meet Server: Video conferencing core
├── Jitsi Videobridge: WebRTC media server
├── Prosody XMPP Server: Signaling and presence
├── PostgreSQL Database: User and meeting data
├── Redis Cache: Session and temporary data
├── Nginx Reverse Proxy: SSL termination and load balancing
└── Monitoring: Prometheus + Grafana for system monitoring
```

**Infrastructure Requirements:**
- **CPU**: 4-8 cores for 50-100 concurrent participants
- **RAM**: 8-16GB for video processing
- **Storage**: 100-500GB for recordings and logs
- **Bandwidth**: 100-500 Mbps for video streaming
- **SSL Certificate**: Valid SSL certificate for secure connections

### **Scalability Considerations**

**Small Scale (5-20 participants):**
- Single server deployment
- Cost: ~$50-100/month
- Suitable for: Small community meetings, office hours

**Medium Scale (20-50 participants):**
- Multi-server deployment with load balancing
- Cost: ~$100-200/month
- Suitable for: Regular community meetings, educational sessions

**Large Scale (50-100 participants):**
- Distributed deployment with multiple videobridges
- Cost: ~$200-500/month
- Suitable for: Large community events, governance meetings

**Enterprise Scale (100+ participants):**
- Multi-region deployment with CDN
- Cost: ~$500+/month
- Suitable for: Special events, external presentations

### **Security Architecture**

**Network Security:**
- **SSL/TLS**: End-to-end encryption for all communications
- **Firewall**: Restrict access to necessary ports only
- **VPN**: Optional VPN for additional security
- **DDoS Protection**: Protection against distributed attacks

**Application Security:**
- **JWT Authentication**: Secure token-based authentication
- **Rate Limiting**: Prevent abuse and resource exhaustion
- **Input Validation**: Sanitize all user inputs
- **Regular Updates**: Keep all components updated

**Data Security:**
- **Encryption at Rest**: Encrypt stored recordings and data
- **Access Control**: Role-based access to meeting data
- **Audit Logging**: Comprehensive logging for security monitoring
- **Backup Security**: Encrypted backups of critical data

## Implementation Strategy

### **Direct Internet Identity Integration**

**Recommended Approach**: Self-hosted Jitsi with direct Internet Identity integration, eliminating the need for intermediate authentication layers or vendor dependencies.

### **Simplified Architecture**

**Authentication Flow:**
```
CPP Community Platform → Internet Identity → Jitsi Meeting
├── User logs into CPP platform via Internet Identity
├── User clicks "Join Meeting" or "Create Meeting"
├── II provides OIDC tokens directly to Jitsi
├── Jitsi validates tokens and creates/joins meeting
├── User enters meeting with CPP identity preserved
└── Meeting analytics linked to CPP user account
```

### **Implementation Timeline**

**Phase 1: Foundation (Weeks 1-4)**
- [ ] Set up self-hosted Jitsi infrastructure
- [ ] Implement direct Internet Identity OIDC integration
- [ ] Create meeting room management system
- [ ] Integrate with CPP user authentication
- [ ] Basic meeting creation and joining

**Phase 2: Community Integration (Weeks 5-8)**
- [ ] Integrate with CPP meetup calendar
- [ ] Implement access control based on bundle sponsorship
- [ ] Add meeting recording and storage
- [ ] Create meeting analytics dashboard
- [ ] Implement notification system integration

**Phase 3: Educational Features (Months 1-2)**
- [ ] Course session integration
- [ ] Breakout room functionality
- [ ] Whiteboard and document sharing
- [ ] Office hours scheduling system
- [ ] Content recording and replay

**Phase 4: Advanced Features (Months 3-6)**
- [ ] Governance meeting integration
- [ ] Voting and polling features
- [ ] Advanced analytics and reporting
- [ ] Mobile app optimization

## Cost Analysis

### **Infrastructure Costs**

| Component           | Small Scale (5-20)                | Medium Scale (20-50)                | Large Scale (50-100)                 |
| ------------------- | --------------------------------- | ----------------------------------- | ------------------------------------ |
| **Server**          | $50/month (4 CPU, 8GB RAM, 100GB) | $150/month (8 CPU, 16GB RAM, 250GB) | $300/month (16 CPU, 32GB RAM, 500GB) |
| **Bandwidth**       | $20/month (100 Mbps)              | $50/month (250 Mbps)                | $100/month (500 Mbps)                |
| **Load Balancer**   | -                                 | $30/month                           | $50/month                            |
| **CDN**             | -                                 | -                                   | $50/month                            |
| **SSL Certificate** | $10/month                         | $10/month                           | $10/month                            |
| **Monitoring**      | $10/month                         | $20/month                           | $40/month                            |
| **Total**           | **$90/month**                     | **$260/month**                      | **$550/month**                       |

### **Cost Comparison with Commercial Platforms**

| Platform              | Cost per User/Month | 50 Users | 100 Users | 500 Users |
| --------------------- | ------------------- | -------- | --------- | --------- |
| **Jitsi Self-Hosted** | $0                  | $260     | $550      | $2,000    |
| **Zoom Pro**          | $15                 | $750     | $1,500    | $7,500    |
| **Microsoft Teams**   | $6                  | $300     | $600      | $3,000    |
| **Google Workspace**  | $6                  | $300     | $600      | $3,000    |

**Savings with Jitsi:**

| User Count    | Jitsi Cost   | Zoom Cost    | Savings      | Savings % |
| ------------- | ------------ | ------------ | ------------ | --------- |
| **50 users**  | $260/month   | $750/month   | $490/month   | **65%**   |
| **100 users** | $550/month   | $1,500/month | $950/month   | **63%**   |
| **500 users** | $2,000/month | $7,500/month | $5,500/month | **73%**   |

## Risk Assessment

### **Technical Risks**
- **Infrastructure Management**: Requires technical expertise for self-hosting
- **Scalability Challenges**: May struggle with very large meetings (200+ participants)
- **Integration Complexity**: Custom integration with CPP platform
- **Performance Issues**: Video quality may vary based on user bandwidth

### **Mitigation Strategies**
- **Gradual Rollout**: Start with small meetings and scale up
- **Technical Support**: Partner with experienced Jitsi hosting provider
- **Fallback Options**: Maintain integration with commercial platforms for large events
- **Performance Monitoring**: Implement comprehensive monitoring and alerting

### **Operational Risks**
- **Maintenance Overhead**: Regular updates and security patches
- **Support Complexity**: Community support vs commercial support
- **Data Management**: Meeting recording storage and management
- **Compliance Requirements**: GDPR and privacy regulation compliance

### **Mitigation Strategies**
- **Automated Updates**: Implement automated update processes
- **Documentation**: Comprehensive documentation and training
- **Backup Strategy**: Robust backup and disaster recovery
- **Compliance Framework**: Built-in privacy and compliance features

## Success Metrics

### **Technical Metrics**
- **Uptime**: Target 99.9% availability
- **Video Quality**: Target 720p+ for 90% of participants
- **Latency**: Target <200ms for video/audio
- **Concurrent Users**: Support 50-100 concurrent participants

### **User Experience Metrics**
- **Meeting Success Rate**: Target 95% successful meetings
- **User Satisfaction**: Target 4.5/5 average rating
- **Adoption Rate**: Target 60% of community members using video features
- **Support Tickets**: Target <5% of users requiring support

### **Business Metrics**
- **Cost Savings**: Track savings vs commercial platforms
- **Vendor Independence**: Measure reduction in vendor dependencies
- **Community Engagement**: Track meeting participation and engagement
- **Educational Impact**: Measure learning outcomes from video sessions

## Strategic Benefits

### **Vendor Independence**
- **Eliminates BigMES exposure** from commercial video platforms
- **Complete data sovereignty** with Swiss-hosted infrastructure
- **No vendor lock-in** or dependency on external services
- **Full control** over features and customization

### **Community Alignment**
- **Open source solution** matches CPP's community values
- **Transparent technology** with no hidden data collection
- **Community-driven development** and improvement
- **Educational accessibility** without commercial barriers

### **Cost Effectiveness**
- **Predictable costs** based on infrastructure, not users
- **No per-user licensing fees** or hidden costs
- **Scalable pricing** that grows with community needs
- **Long-term cost savings** vs commercial alternatives

### **Integration Benefits**
- **Seamless authentication** with Internet Identity
- **Unified user experience** across CPP platform
- **Rich analytics** and community insights
- **Custom features** tailored to CPP needs

## Conclusion

**Jitsi integration provides the optimal video conferencing solution for CPP:**

✅ **Perfect Alignment**: Matches all CPP strategic goals and values
✅ **Technical Excellence**: 90% of commercial platform capabilities
✅ **Cost Effectiveness**: Significant savings vs commercial platforms
✅ **Vendor Independence**: Complete elimination of BigMES exposure
✅ **Community Integration**: Seamless integration with CPP platform
✅ **Future-Proof**: Scalable solution that grows with community needs

**Implementation Recommendation:**
- **Phase 1**: Start with self-hosted Jitsi for small community meetings
- **Phase 2**: Scale up to support educational content delivery
- **Phase 3**: Expand to governance and large community events
- **Phase 4**: Optimize and add advanced features

**Bottom Line**: Jitsi provides the perfect balance of capability, independence, and cost-effectiveness for CPP's video conferencing needs while maintaining complete alignment with strategic goals.

---

**Related Documents:**
- [Community Strategy](community-strategy.md) - Overall community platform architecture
- [Platform Comparison](platform-comparison.md) - Authentication infrastructure analysis
- [Internet Identity Documentation](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/) - Official II documentation
- [Jitsi Meet Documentation](https://jitsi.github.io/handbook/) - Official Jitsi documentation 

### **Encryption and Security Considerations**

#### **Jitsi Security Features**

**Standard Encryption:**
- **Media Encryption**: AES-256-GCM (256-bit key)
- **Signaling Encryption**: TLS 1.3
- **Key Exchange**: DTLS-SRTP with perfect forward secrecy
- **Security Level**: **Very Strong** - enterprise-grade encryption

**E2E Encryption:**
- **Status**: Available and fully supported
- **Implementation**: Open source WebRTC E2E protocol
- **Compatibility**: All modern browsers and clients
- **Auditability**: Open source, full transparency

#### **Security Advantages**

| Aspect                  | Self-Hosted Jitsi | Benefits                            |
| ----------------------- | ----------------- | ----------------------------------- |
| **Encryption Strength** | AES-256-GCM       | Enterprise-grade security           |
| **E2E Availability**    | ✅ Full            | Complete end-to-end encryption      |
| **Code Transparency**   | ✅ Open Source     | Full auditability and customization |
| **Data Sovereignty**    | ✅ CPP Control     | Complete data ownership             |
| **Auditability**        | ✅ Full            | Complete transparency and control   |

## PWA Integration Considerations

### **Push Notifications**

**Meeting Notifications:**
- **Meeting Reminders**: 15-minute, 1-hour, and 1-day advance notifications
- **Meeting Updates**: Changes to meeting time, location, or agenda
- **Meeting Cancellations**: Immediate notification of cancelled meetings
- **Join Reminders**: Direct "Join Meeting" action buttons in notifications

**Technical Implementation:**
```javascript
// PWA Service Worker for Jitsi notifications
const jitsiNotificationService = {
  // Meeting reminder notifications
  scheduleMeetingReminder: (meeting, reminderTime) => {
    const notification = {
      title: `CPP Meeting Reminder: ${meeting.title}`,
      body: `Meeting starts in ${reminderTime}`,
      icon: '/icons/cpp-logo.png',
      badge: '/icons/meeting-badge.png',
      data: {
        meetingId: meeting.id,
        action: 'join_meeting',
        url: `/meetings/${meeting.id}`
      },
      actions: [
        {
          action: 'join',
          title: 'Join Meeting',
          icon: '/icons/join-meeting.png'
        },
        {
          action: 'dismiss',
          title: 'Dismiss',
          icon: '/icons/dismiss.png'
        }
      ]
    };
    
    return self.registration.showNotification(notification);
  },
  
  // Handle notification clicks
  handleNotificationClick: (event) => {
    if (event.action === 'join') {
      // Open Jitsi meeting directly
      event.notification.close();
      clients.openWindow(event.notification.data.url);
    }
  }
};
```

**Notification Permissions:**
- **Opt-in Strategy**: Request permissions during first meeting creation
- **Granular Control**: Allow users to choose notification types
- **Privacy Compliance**: GDPR-compliant notification preferences
- **Cross-platform**: Works on mobile and desktop browsers

### **Calendar Integration**

**Independent PWA Calendar Function:**
- **Meeting Management**: Create, edit, and delete meetings
- **Recurring Meetings**: Support for weekly/monthly recurring sessions
- **Time Zone Handling**: Automatic timezone conversion for global community
- **Calendar Sync**: Export to Google Calendar, Outlook, iCal
- **Bundle-based Access**: Meeting visibility based on bundle sponsorship

**Calendar Features:**
```javascript
// PWA Calendar Component
const cppCalendar = {
  // Meeting creation with Jitsi integration
  createMeeting: (meetingData) => {
    const meeting = {
      id: generateMeetingId(),
      title: meetingData.title,
      description: meetingData.description,
      startTime: meetingData.startTime,
      endTime: meetingData.endTime,
      timezone: meetingData.timezone,
      accessLevel: meetingData.accessLevel, // public, members-only, sponsors-only
      bundleId: meetingData.bundleId,
      jitsiRoom: `cpp-${meetingData.bundleId}-${meetingData.id}`,
      participants: meetingData.participants,
      notifications: meetingData.notifications
    };
    
    // Create Jitsi room
    jitsiAPI.createRoom(meeting.jitsiRoom);
    
    // Schedule notifications
    scheduleMeetingNotifications(meeting);
    
    // Add to calendar
    return calendarAPI.addMeeting(meeting);
  },
  
  // Calendar view with meeting integration
  renderCalendar: (month, year) => {
    const meetings = calendarAPI.getMeetings(month, year);
    const calendarView = meetings.map(meeting => ({
      ...meeting,
      jitsiUrl: `/meetings/${meeting.id}`,
      canJoin: isUserAuthorized(meeting.accessLevel, meeting.bundleId),
      joinButton: meeting.startTime <= now && meeting.endTime >= now
    }));
    
    return renderCalendarGrid(calendarView);
  }
};
```

**Calendar Synchronization:**
- **Export Formats**: iCal, Google Calendar, Outlook
- **Real-time Updates**: Live sync with meeting changes
- **Privacy Controls**: User controls what gets exported
- **Bundle Integration**: Meeting visibility based on user's bundle access

### **PWA-Specific Considerations**

**Offline Functionality:**
- **Calendar Caching**: Offline access to meeting schedules
- **Meeting Details**: Cached meeting information when offline
- **Sync on Reconnect**: Automatic sync when connection restored
- **Offline Notifications**: Local notification scheduling

**Mobile Optimization:**
- **Touch Interface**: Optimized for mobile touch interactions
- **Responsive Design**: Adaptive layout for different screen sizes
- **Mobile Notifications**: Native mobile notification support
- **Background Sync**: Background meeting data synchronization

**Performance Considerations:**
- **Lazy Loading**: Load meeting details on demand
- **Image Optimization**: Optimized meeting thumbnails and avatars
- **Caching Strategy**: Intelligent caching of calendar data
- **Bundle Splitting**: Separate calendar and video components

### **Integration Architecture**

**PWA + Jitsi Integration Points:**
```
CPP PWA Components:
├── Calendar Module (Independent)
│   ├── Meeting Management
│   ├── Timezone Handling
│   ├── Bundle-based Access
│   └── Export/Sync Features
├── Notification Service
│   ├── Push Notifications
│   ├── Meeting Reminders
│   └── Action Handlers
├── Jitsi Integration
│   ├── Direct Meeting Launch
│   ├── Room Management
│   └── User Authentication
└── Bundle Management
    ├── Access Control
    ├── Meeting Permissions
    └── Community Features
```

**Data Flow:**
1. **Calendar Creation**: User creates meeting in PWA calendar
2. **Jitsi Room**: PWA automatically creates corresponding Jitsi room
3. **Notifications**: PWA schedules push notifications for meeting
4. **Access Control**: Bundle-based permissions determine meeting access
5. **Meeting Launch**: Direct launch from calendar to Jitsi meeting
6. **Analytics**: Meeting participation tracked back to CPP platform