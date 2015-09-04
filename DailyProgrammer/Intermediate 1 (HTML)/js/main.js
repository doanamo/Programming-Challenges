$("#addEventButton").click(function()
{
    // Show the modal dialog.
    $("#addEventModal").modal("show");
});

$("#addEventConfirmButton").click(function()
{
    // Get form input values.
    var form = $("#addEventForm");

    var date = form.find("#inputDate").val();
    var time = form.find("#inputTime").val();
    var description = form.find("#inputDescription").val();

    // Add a new event.
    addEvent(date, time, description);

    // Save event list.
    saveEvents();

    // Hide the modal dialog.
    $("#addEventModal").modal("hide");

    // Clear the dialog form.
    $("#addEventForm")[0].reset();
});

$("#editEventConfirmButton").click(function()
{
    // Get form input values.
    var form = $("#editEventForm");
    var date = form.find("#inputDate").val();
    var time = form.find("#inputTime").val();
    var description = form.find("#inputDescription").val();

    // Edit element fields.
    var element = $("#editEventModal").data("event");
    element.find(".date").text(date);
    element.find(".time").text(time);
    element.find(".description").text(description);

    // Save event list.
    saveEvents();

    // Hide the modal dialog.
    $("#editEventModal").modal("hide");
});

$("#editEventDeleteButton").click(function()
{
    // Remove event element.
    $("#editEventModal").data("event").remove();

    // Save event list.
    saveEvents();

    // Hide the modal dialog.
    $("#editEventModal").modal("hide");
});

$("#editEventModal").on("hidden.bs.modal", function()
{
    // Remove event reference.
    $("#editEventModal").data("event", null);
});

function addEvent(date, time, description)
{
    // Create an edit button.
    var editButton = $("<span>");
    editButton.attr("id", "editEventButton");
    editButton.addClass("glyphicon glyphicon-edit");
    editButton.click(function()
    {
        // Show the modal dialog.
        $("#editEventModal").modal("show");

        // Get the event element.
        var element = editButton.data("event");

        // Fill form input elements.
        var form = $("#editEventForm");
        form.find("#inputDate").val(element.find(".date").text());
        form.find("#inputTime").val(element.find(".time").text());
        form.find("#inputDescription").val(element.find(".description").text());

        // Set currently edited event.
        $("#editEventModal").data("event", element);
    });

    // Create a list element.
    var element = $("<tr>");
    element.append($("<td>").addClass("date").text(date));
    element.append($("<td>").addClass("time").text(time));
    element.append($("<td>").addClass("description").text(description));
    element.append($("<td>").addClass("button").append(editButton));
    editButton.data("event", element);

    // Append element to the list.
    $("#eventList").append(element);
}

function saveEvents()
{
    // Clear event list.
    localStorage.removeItem("events");

    // Store all events in an array.
    var events = [];

    $("#eventList").find("tr").each(function()
    {
        var entry = {};
        entry.date = $(this).find(".date").text();
        entry.time = $(this).find(".time").text();
        entry.description = $(this).find(".description").text();
        events.push(entry);
    });

    // Write to local storage.
    localStorage.setItem("events", JSON.stringify(events));
}

function loadEvents()
{
    // Get events from local storage.
    var events = JSON.parse(localStorage.getItem("events"));

    // Add every event.
    for(var i in events)
    {
        var entry = events[i];
        addEvent(entry.date, entry.time, entry.description);
    }
}

// Query local storage.
if(localStorage.getItem("events") == null)
{
    // Add example events.
    addEvent("2015-09-01", "08:00", "Go out and have some fun at the city centre.");
    addEvent("2015-09-02", "14:30", "Drink some fine wine and get wasted.");
    addEvent("2015-09-03", "21:13", "Find the person you've been looking for your entire life.");

    // Save events.
    saveEvents();
}
else
{
    // Load events.
    loadEvents();
}
