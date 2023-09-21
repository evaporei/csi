#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>
#include <unistd.h>

#define DNS_PORT 53
#define BUFFER_SIZE 512

// Simple header structure for DNS (following RFC1035)
struct DNS_HEADER {
    unsigned short id;
    unsigned short flags;
    unsigned short q_count;
    unsigned short ans_count;
    unsigned short auth_count;
    unsigned short add_count;
};

// Question type
struct QUESTION {
    unsigned short qtype;
    unsigned short qclass;
};

// Define the Resource Record (RR) structure
struct R_DATA {
    unsigned short rtype;
    unsigned short rclass;
    unsigned int ttl;
    unsigned short data_len;
};

// Function to convert DNS labels format (www3google3com0) to human readable format (www.google.com)
void format_name(unsigned char *reader, unsigned char *buffer) {
    unsigned char *name;
    unsigned int offset;
    int jumped = 0, i , j;

    name = (unsigned char*)malloc(256);

    i = 0;
    j = 0;

    // Read the names in 3www6google3com format
    while (*reader != 0) {
        if (*reader >= 192) {
            offset = (*reader)*256 + *(reader+1) - 49152; // 49152 = 11000000 00000000 ;)
            reader = buffer + offset - 1;
            jumped = 1; // We have jumped to another location so counting wont go up!
        } else {
            name[j++] = *reader;
        }

        reader = reader + 1;

        if (jumped == 0) {
            i++;
        }
    }

    name[j] = '\0'; // Null terminate the array

    if (jumped == 1) {
        i++;
    }

    // Now convert 3www6google3com0 to www.google.com
    for (i = 0; i < (int)strlen((const char*)name); i++) {
        int len = name[i];
        for (j = 0; j < len; j++) {
            printf("%c", name[i+j+1]);
        }

        if (name[i+len+1] != 0) {
            printf(".");
        }

        i += len;
    }

    free(name);
}

int main(void) {
    int sockfd;
    struct sockaddr_in server_addr;
    char buffer[BUFFER_SIZE];
    struct DNS_HEADER *dns_header;
    struct QUESTION *question;
    char *qname;

    // Create socket
    sockfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (sockfd < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(DNS_PORT);
    inet_pton(AF_INET, "8.8.8.8", &server_addr.sin_addr);

    // Construct DNS query
    memset(buffer, 0, BUFFER_SIZE);

    dns_header = (struct DNS_HEADER *) buffer;
    dns_header->id = htons(12345); // Random ID
    dns_header->flags = htons(0x0100); // Standard query
    dns_header->q_count = htons(1); // One question

    qname = (char *)(dns_header + 1); // Pointing just after the DNS header

    // Convert "www.example.com" into "3www7example3com0"
    char domain[] = "www.example.com";
    char *token = strtok(domain, ".");
    while (token != NULL) {
        size_t len = strlen(token);
        *qname++ = len;
        memcpy(qname, token, len);
        qname += len;

        token = strtok(NULL, ".");
    }
    *qname++ = 0; // Null terminate

    question = (struct QUESTION *)qname;
    question->qtype = htons(1);  // A record
    question->qclass = htons(1); // Internet class

    // Send DNS query
    if (sendto(sockfd, buffer, sizeof(struct DNS_HEADER) + (qname - (char *)(dns_header + 1)) + sizeof(struct QUESTION), 0,
               (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("sendto failed");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    printf("Sent DNS query.\n");

    // Get response
    struct sockaddr_in response_addr;
    socklen_t addrlen = sizeof(response_addr);
    int n = recvfrom(sockfd, buffer, BUFFER_SIZE, 0, (struct sockaddr *)&response_addr, &addrlen);

    if (n < 0) {
        perror("recvfrom failed");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    dns_header = (struct DNS_HEADER *) buffer;

    unsigned short q_count = ntohs(dns_header->q_count);
    unsigned short ans_count = ntohs(dns_header->ans_count);

    printf("Questions: %d, Answers: %d\n", q_count, ans_count);

    qname = (char *)(dns_header + 1);

    // Print the question name
    printf("Query: ");
    format_name((unsigned char *)qname, (unsigned char *)buffer);
    printf("\n");

    // Adjust the qname pointer to point after the QUESTION section
    while (*qname) qname++;
    qname += sizeof(struct QUESTION) + 1; // +1 for the 0 byte at the end of the qname

    // Parsing the answer section
    for (int i = 0; i < ans_count; i++) {
        struct R_DATA *r_data;
        r_data = (struct R_DATA *)qname;
        qname += sizeof(struct R_DATA);

        if (ntohs(r_data->rtype) == 1) { // A record
            struct in_addr addr;
            memcpy(&addr, qname, sizeof(addr));
            printf("Answer: %s has IPv4 address %s\n", domain, inet_ntoa(addr));
        }

        qname += ntohs(r_data->data_len);
    }

    close(sockfd);
    return 0;
}

