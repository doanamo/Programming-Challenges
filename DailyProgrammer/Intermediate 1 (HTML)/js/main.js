$("#addEventButton").click(function()
{
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

    // Hide the modal dialog.
    $("#addEventModal").modal("hide");

    // Clear the dialog form.
    $("#addEventForm")[0].reset();
});

function addEvent(date, time, description)
{
    // Create an edit button.
    var editButton = $("<span>");
    editButton.attr("id", "editEventButton");
    editButton.addClass("glyphicon glyphicon-edit");
    editButton.click(function()
    {
        $("#editEventModal").modal("show");
    });

    // Create a list element.
    var element = $("<tr>");
    element.append($("<td>").text(date));
    element.append($("<td>").text(time));
    element.append($("<td>").text(description));
    element.append($("<td>").append(editButton));

    // Append element to the list.
    $("#eventList").append(element);
}

// Add example events.
addEvent("2015-09-01", "08:00", "Go out and have some fun at the city centre.");
addEvent("2015-09-02", "14:30", "Drink some fine wine and get wasted.");
addEvent("2015-09-03", "21:13", "Find the person you've been looking for your entire life.");
