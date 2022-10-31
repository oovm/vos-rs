

/// Update an existing pet
#get("/header", 200)
def ExampleService.Create(Pet) -> String {
    #404 #FormData
    return A;
}

#get("/header")
def ExampleService.Create(Pet) -> OK {
    #400 return Json;
    #404 return !;
}

def ExampleService.Create(EchoRequest) -> EchoResponse;
