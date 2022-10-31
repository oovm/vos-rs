


```proto
service NoteService {
    rpc Fetch(NoteFilter) returns (Notebook) {
        option (google.api.http) = {
            get: "/api/v1/notes"
        };
    };
    rpc Create(Note) returns (Notebook) {
        option (google.api.http) = {
            post: "/api/v1/notes"
            body: "*"
        };
    };
}
```


```

#post(a)
def note.service.fetch(node: NoteFilter) -> Notebook {
    http: {
        post: "",
        body: ""
    }
}


service libiary A {
    
    
}

service clinet {
    a: a
}
```